# mir
`ls` alternative.

## Usage

`mir` is a simple command-line tool that lists files and directories in the
current directory, similar to the `ls` command.

### Basic Usage

1. Navigate to any directory:
   ```bash
   cd /path/to/directory
   ```

2. Run `mir` to list all files and directories:
   ```bash
   mir
   ```

The command will display all files and directories in the current location,
including hidden files (those starting with `.`).

### Features

- Lists all files and directories in the current directory
- Shows hidden files (starting with `.`)
- Simple and straightforward output

### Example Output

```bash
$ mir
.git/
.gitignore
Cargo.toml
README.md
src/
target/
```

## Installation

You can install `mir` using Cargo, current stable version is `v1.0.1`.

```bash
cargo install --git https://github.com/fitiavana07/mir.git --tag v1.0.1
```


## v1.0.0 checklist

- [x] 📄 Add usage documentation in `README.md`
- [x] 📂 List all files and directories in the current directory, including hidden ones (starting with `.`)
- [x] 📂 Distinguish directories by suffixing `/`
- [x] ⚠️ Gracefully handle edge cases:
  - [x] Directory not found (e.g. deleted)
  - [x] Permission denied
  - [x] Empty directory
  - [x] Large number of files
  - [x] Filenames with non-ASCII characters (UTF-8 support)

- [x] 🛠️ Installable via `cargo install --git <repo-url>` and installation
  documentation in `README.md`, allowing to specify a git tag
- [x] 🔓 Make the repository public
- [x] 🏷️ Create `v1.0.0` git tag

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.
