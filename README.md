# mir
`ls` alternative.

### v1.0.0 checklist

- [ ] 📄 Add usage documentation in `README.md`
- [ ] 📂 List all files and directories in the current directory, including hidden ones (starting with `.`)
- [ ] 📂 Distinguish directories by suffixing `/`
- [ ] ⚠️ Gracefully handle edge cases:
      - Directory not found (e.g. deleted)
      - Permission denied
      - Empty directory
      - Large number of files
      - Filenames with non-ASCII characters (UTF-8 support)
- [ ] 🛠️ Installable via `cargo install --git <repo-url>` and installation documentation in `README.md`
- [ ] 🔓 Make the repository public
- [ ] 🏷️ Create `v1.0.0` git tag
