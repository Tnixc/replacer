# Replacer

CLI tool to replace strings in all files in a directory (recursively) (It's written in rust and blazingly fast)

# Usage

You will need a `config.toml` in the following format, note that it is case sensitive:
```toml
# config.toml
from = "to"
```
Then run the following:
```sh
replacer ./path/to/target
```

# Install
```sh
cargo install --git https://github.com/Tnixc/replacer
```
