# Replacer

CLI tool to replace strings in all files in a directory (recursively) (It's written in rust and blazingly fast)

# Usage

You will need a `replacer.config.toml` in the following format:
```toml
# replacer.config.toml
papairs = [ # Must be present.
    ["old text 1", "new text 1"],
    ["foo", "bar"]
]

case_sensitive = true # defaults to true

# Ignore Configuration
ignore = [".git", "dist", ".env", "ignore_this.txt", "*.ignore"]
```

Then run the following:
```sh
replacer ./path/to/target
```

# Install
```sh
cargo install replacer-cli
```
