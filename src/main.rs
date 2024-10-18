use color_print::cprintln;
use regex::Regex;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
mod args;

#[derive(Debug, Deserialize)]
struct Config {
    pairs: Vec<(String, String)>,
    #[serde(default)]
    ignore: Vec<String>,
    case_sensitive: Option<bool>,
}

#[derive(Debug)]
struct Replacement {
    pub from: Regex,
    pub to: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = args::args();

    let target = match matches.get_one::<String>("target") {
        Some(t) => t,
        None => {
            cprintln!("<b><r>Error</></>: Target argument is missing");
            return Ok(());
        }
    };

    let config_path = match matches.get_one::<String>("config") {
        Some(c) => c,
        None => {
            cprintln!("<b><r>Error</></>: Config argument is missing");
            return Ok(());
        }
    };

    let case_sensitive = matches.get_flag("case-insensitive");

    let config_content = match fs::read_to_string(config_path) {
        Ok(content) => content,
        Err(e) => {
            cprintln!(
                "<b><r>Error</></>: Failed to read config file '{}': {}",
                config_path,
                e
            );
            return Ok(());
        }
    };

    let mut config: Config = match toml::from_str(&config_content) {
        Ok(c) => c,
        Err(e) => {
            cprintln!("<b><r>Error</></>: Failed to parse config file: {}", e);
            return Ok(());
        }
    };

    // Update ignore config based on command-line arguments
    if let Some(ignore_paths) = matches.get_many::<String>("ignore") {
        config.ignore.extend(ignore_paths.cloned());
    }

    // Handle no-ignore paths
    if let Some(no_ignore_paths) = matches.get_many::<String>("no-ignore") {
        let no_ignore_set: HashSet<_> = no_ignore_paths.collect();
        config.ignore.retain(|x| !no_ignore_set.contains(x));
    }

    let mut pairs_map: HashMap<String, String> = config.pairs.into_iter().collect();
    if let Some(cli_pairs) = matches.get_many::<(String, String)>("pair") {
        for (from, to) in cli_pairs {
            pairs_map.insert(from.to_string(), to.to_string());
        }
    }

    config.pairs = pairs_map.into_iter().collect();

    if config.pairs.is_empty() {
        cprintln!(
            "<b><r>Error</></>: No pairs found in the config file or command-line arguments."
        );
        return Ok(());
    }

    config.ignore.push(config_path.to_string());

    if !config.ignore.contains(&".git".to_string()) {
        config.ignore.push(".git".to_string());
    }

    let case_sensitive = config.case_sensitive.unwrap_or(case_sensitive);

    let replacements: Vec<Replacement> = match config
        .pairs
        .into_iter()
        .map(|(from, to)| {
            let regex_string = if case_sensitive {
                from
            } else {
                format!("(?i){}", from)
            };
            match Regex::new(&regex_string) {
                Ok(regex) => Ok(Replacement { from: regex, to }),
                Err(e) => {
                    cprintln!("<b><r>Error</></>: Invalid regex '{}': {}", regex_string, e);
                    Err(())
                }
            }
        })
        .collect::<Result<Vec<_>, ()>>()
    {
        Ok(r) => r,
        Err(_) => return Ok(()),
    };

    let ignore_patterns: Vec<Regex> = match config
        .ignore
        .iter()
        .filter(|&pattern| pattern.contains('*') || pattern.contains('?'))
        .map(|pattern| match Regex::new(&glob_to_regex(pattern)) {
            Ok(regex) => Ok(regex),
            Err(e) => {
                cprintln!(
                    "<b><r>Error</></>: Invalid glob pattern '{}': {}",
                    pattern,
                    e
                );
                Err(())
            }
        })
        .collect::<Result<Vec<_>, ()>>()
    {
        Ok(p) => p,
        Err(_) => return Ok(()),
    };

    if Path::new(target).is_dir() {
        recursive_file(
            &PathBuf::from(target),
            &replacements,
            &config.ignore,
            &ignore_patterns,
        );
    } else if Path::new(target).is_file() {
        op(
            &PathBuf::from(target),
            &replacements,
            &config.ignore,
            &ignore_patterns,
        );
    } else {
        cprintln!(
            "<b><r>Error</></>: The specified path '{}' is neither a file nor a directory.",
            target
        );
    }

    Ok(())
}

fn recursive_file(
    path: &PathBuf,
    replacements: &[Replacement],
    ignore: &[String],
    ignore_patterns: &[Regex],
) {
    if ignore.contains(
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
                recursive_file(&path, replacements, ignore, ignore_patterns);
            } else if path.is_file() {
                op(&path, replacements, ignore, ignore_patterns);
            }
        }
    }
}

fn op(file: &PathBuf, reqs: &[Replacement], ignore: &[String], ignore_patterns: &[Regex]) {
    let file_name = file.file_name().unwrap().to_string_lossy();

    if ignore.contains(&file_name.to_string())
        || ignore_patterns
            .iter()
            .any(|pattern| pattern.is_match(&file_name))
    {
        return;
    }

    if !file.is_file() {
        cprintln!("<b><r>Error</></>: Operation failed at {}.", file.display());
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
                cprintln!(
                    "<b><r>Error</></>: Failed to write to {}: {}",
                    file.display(),
                    e
                );
            } else {
                if replaced {
                    cprintln!("<b><g>Replaced</></>: {:?}", file);
                } else {
                    cprintln!("<b><y>No change</></>: {:?}", file);
                }
            }
        }
        Err(e) => cprintln!(
            "<b><r>Error</></>: Failed to read {}: {}",
            file.display(),
            e
        ),
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
