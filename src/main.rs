use regex::Regex;
use serde::Deserialize;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
struct Config {
    pairs: Vec<(String, String)>,
    #[serde(default)]
    ignore: IgnoreConfig,
    case_sensitive: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct IgnoreConfig {
    files: Vec<String>,
    directories: Vec<String>,
    patterns: Vec<String>,
}

impl Default for IgnoreConfig {
    fn default() -> Self {
        IgnoreConfig {
            files: vec![],
            directories: vec![],
            patterns: vec![],
        }
    }
}

#[derive(Debug)]
struct Replacement {
    pub from: Regex,
    pub to: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <path/to/target>", args[0]);
        return Ok(());
    }
    let query = &args[1];
    let mut config: Config = toml::from_str(&fs::read_to_string("./replacer.config.toml")?)?;
    if config.pairs.is_empty() {
        println!("No pairs found in the config file.");
        return Ok(());
    }

    config.ignore.files.push("replacer.config.toml".to_string());

    let case_sensitive = config.case_sensitive.unwrap_or(true);

    let replacements: Vec<Replacement> = config
        .pairs
        .into_iter()
        .map(|(from, to)| {
            let regex_string = if case_sensitive {
                from
            } else {
                format!("(?i){}", from)
            };
            Replacement {
                from: Regex::new(&regex_string).unwrap(),
                to,
            }
        })
        .collect();

    let ignore_patterns: Vec<Regex> = config
        .ignore
        .patterns
        .clone()
        .into_iter()
        .map(|pattern| Regex::new(&glob_to_regex(&pattern)).unwrap())
        .collect();

    if Path::new(query).is_dir() {
        recursive_file(
            &PathBuf::from(query),
            &replacements,
            &config.ignore,
            &ignore_patterns,
        );
    } else if Path::new(query).is_file() {
        op(
            &PathBuf::from(query),
            &replacements,
            &config.ignore,
            &ignore_patterns,
        );
    } else {
        eprintln!("The specified path is neither a file nor a directory.");
    }

    Ok(())
}

fn recursive_file(
    path: &PathBuf,
    replacements: &[Replacement],
    ignore_config: &IgnoreConfig,
    ignore_patterns: &[Regex],
) {
    if ignore_config.directories.contains(
        &path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string(),
    ) {
        return;
    }

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                recursive_file(&path, replacements, ignore_config, ignore_patterns);
            } else if path.is_file() {
                op(&path, replacements, ignore_config, ignore_patterns);
            }
        }
    }
}

fn op(
    file: &PathBuf,
    reqs: &[Replacement],
    ignore_config: &IgnoreConfig,
    ignore_patterns: &[Regex],
) {
    let file_name = file.file_name().unwrap().to_string_lossy();

    // Check if the file should be ignored
    if ignore_config.files.contains(&file_name.to_string())
        || ignore_patterns
            .iter()
            .any(|pattern| pattern.is_match(&file_name))
    {
        return;
    }

    if !file.is_file() {
        eprintln!("Error: Operation failed at {}.", file.display());
        return;
    }

    match fs::read_to_string(file) {
        Ok(mut text) => {
            let old = text.clone();
            for req in reqs {
                text = req.from.replace_all(&text, &req.to).to_string();
            }
            let mut replaced = false;
            if old != text {
                replaced = true;
            }
            if let Err(e) = fs::write(file, text) {
                eprintln!("Failed to write to {}: {}", file.display(), e);
            } else {
                if replaced {
                    println!("Replaced: {:?}", file);
                } else {
                    println!("No change: {:?}", file);
                }
            }
        }
        Err(e) => eprintln!("Failed to read {}: {}", file.display(), e),
    }
}

fn glob_to_regex(pattern: &str) -> String {
    let mut regex = String::new();
    regex.push('^');
    for c in pattern.chars() {
        match c {
            '*' => regex.push_str(".*"),
            '?' => regex.push('.'),
            '.' => regex.push_str("\\."),
            _ => regex.push(c),
        }
    }
    regex.push('$');
    regex
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_glob_to_regex() {
        assert_eq!(glob_to_regex("*.txt"), "^.*\\.txt$");
        assert_eq!(glob_to_regex("file?.log"), "^file.\\.log$");
        assert_eq!(glob_to_regex("data.*"), "^data\\..*$");
    }

    #[test]
    fn test_replacement() {
        let replacement = Replacement {
            from: Regex::new("(?i)hello").unwrap(),
            to: "world".to_string(),
        };
        let text = "Hello, HELLO, hello";
        let result = replacement.from.replace_all(text, &replacement.to);
        assert_eq!(result, "world, world, world");
    }

    #[test]
    fn test_ignore_patterns() {
        let ignore_patterns = vec![
            Regex::new(&glob_to_regex("*.log")).unwrap(),
            Regex::new(&glob_to_regex("temp*")).unwrap(),
        ];
        assert!(ignore_patterns[0].is_match("file.log"));
        assert!(!ignore_patterns[0].is_match("file.txt"));
        assert!(ignore_patterns[1].is_match("temp_file"));
        assert!(!ignore_patterns[1].is_match("file_temp"));
    }
}
