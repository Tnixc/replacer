use clap::{Arg, ArgMatches, Command};
use std::error::Error;

pub fn args() -> ArgMatches {
    let cmd = Command::new("Replacer")
        .author("Tnixc")
        .about("Replaces text in files, recursively and quickly")
        .arg(
            Arg::new("target")
                .help("Path to the target file or directory")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Path to the config file")
                .default_value("replacer.config.toml")
                .num_args(1),
        )
        .arg(
            Arg::new("case-insensitive")
                .short('s')
                .long("case-insensitive")
                .help("Make case insensitive")
                .action(clap::ArgAction::SetFalse),
        )
        .arg(
            Arg::new("ignore")
                .short('i')
                .long("ignore")
                .value_name("PATHS")
                .help("Ignore directories or files")
                .action(clap::ArgAction::Append)
                .num_args(1..),
        )
        .arg(
            Arg::new("no-ignore")
                .short('n')
                .long("no-ignore")
                .value_name("PATHS")
                .help("Don't ignore directories or files (overrides ignore config)")
                .action(clap::ArgAction::Append)
                .num_args(1..),
        )
        .arg(
            Arg::new("pair")
                .short('p')
                .long("pair")
                .value_name("KEY=VALUE")
                .help("Additional replacement pairs (format: key=value). You can pass multiple, such as `-p \"foo=bar\" -p\"old=new\"`")
                .value_parser(parse_pair)
                .action(clap::ArgAction::Append)
                .num_args(1),
        )
        .after_help(r#"
Flags passed to cli always override those set in the config file.
You do not need a config file if you pass pairs as flags.
HINT: You can use -- after flags which take in PATHS (such as -i) to continue the args.
"#)
        .get_matches();
    return cmd;
}

pub fn parse_pair(s: &str) -> Result<(String, String), Box<dyn Error + Send + Sync + 'static>> {
    let pos = s
        .find('=')
        .ok_or_else(|| format!("Invalid key=value format: no `=` found in `{s}`"))?;
    Ok((s[..pos].trim().to_string(), s[pos + 1..].trim().to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pair() {
        assert!(parse_pair("invalid").is_err());
    }
}
