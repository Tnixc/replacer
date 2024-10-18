# Replacer

A BLAZINGLy FAST CLI tool written in Rust to replace strings in files or stdin, recursively and quickly.

## Features

- Replace text in files or directories recursively
- Process input from stdin
- Case-sensitive or case-insensitive replacements
- Ignore specific files or directories
- Use glob patterns for ignoring files
- Override ignore settings
- Specify replacement pairs via CLI or config file

## Installation

```sh
cargo install replacer-cli
```

## Usage

```sh
replacer [FLAGS] [OPTIONS] [TARGET]
```

If no TARGET is specified, Replacer will process input from stdin and write to stdout.

### Flags

- `-s, --case-insensitive`: Make replacements case-insensitive
- `-h, --help`: Print help information
- `-V, --version`: Print version information

### Options

- `-c, --config <FILE>`: Path to the config file (default: "replacer.config.toml")
- `-i, --ignore <PATHS>...`: Ignore directories or files
- `-n, --no-ignore <PATHS>...`: Don't ignore directories or files (overrides ignore config)
- `-p, --pair <KEY=VALUE>`: Additional replacement pairs (format: key=value)

### Arguments

- `<TARGET>`: Path to the target file or directory (omit for stdin)

## Configuration File

You can use a configuration file (default: `replacer.config.toml`) in the following format:

```toml
# replacer.config.toml
pairs = [
    ["old text 1", "new text 1"],
    ["foo", "bar"]
]

case_sensitive = true  # Optional, defaults to true

# Ignore Configuration (Optional)
ignore = [".git", "dist", ".env", "ignore_this.txt", "*.ignore"]
```

## Examples

1. Replace text in a directory using a config file:

   ```sh
   replacer ./path/to/target
   ```

2. Replace text from stdin:

   ```sh
   echo "Hello, world!" | replacer
   ```

3. Use CLI arguments for replacement pairs:

   ```sh
   replacer -p "foo=bar" -p "old=new" ./path/to/target
   ```

4. Ignore specific directories:

   ```sh
   replacer -i node_modules -i .git ./path/to/target
   ```

5. Override ignore settings:

   ```sh
   replacer -n important_file.txt ./path/to/target
   ```

6. Case-insensitive replacement:
   ```sh
   replacer -s -p "HELLO=world" ./path/to/target
   ```

## Tips

- Flags passed to CLI always override those set in the config file.
- You don't need a config file if you pass pairs as flags.
- Use `--` after flags which take in PATHS (such as -i) to continue the args.
- The `.git` directory is ignored by default.
- You can use glob patterns in the ignore list, such as `*.log` or `temp*`.

## Notes

- When using stdin, the tool will process the input and write the result to stdout.
- If a file can't be read or written, an error message will be displayed, but the program will continue processing other files.
