# Replacer

CLI tool to replace strings in all files in a directory (recursively) (It's written in rust and blazingly fast)

# Usage

You will need a `replacer.config.toml` in the following format:
```toml
# replacer.config.toml
pairs = [ # must be present
    ["foo", "bar"] 
]
case_sensitive = true # defaults to true

# Ignore Configuration
[ignore]
files = ["ignore_this.txt"] # replacer.config.toml is ignored by default
directories = ["ignore_dir"] # .git is also ignored by default
patterns = ["*.ignore"]
```

Then run the following:
```sh
replacer ./path/to/target
```

# Install
```sh
cargo install replacer-cli
```
