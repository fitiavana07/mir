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

### v1.0.0 checklist

- [x] 📄 Add usage documentation in `README.md`
- [x] 📂 List all files and directories in the current directory, including hidden ones (starting with `.`)
- [x] 📂 Distinguish directories by suffixing `/`
- [ ] ⚠️ Gracefully handle edge cases:
  - [ ] Directory not found (e.g. deleted)
  - [ ] Permission denied
  - [ ] Empty directory
  - [ ] Large number of files
  - [ ] Filenames with non-ASCII characters (UTF-8 support)

- [ ] 🛠️ Installable via `cargo install --git <repo-url>` and installation documentation in `README.md`
- [ ] 🔓 Make the repository public
- [ ] 🏷️ Create `v1.0.0` git tag
