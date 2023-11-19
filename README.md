<div align="center">

# Shikibetsu (識別)

A command-line tool for listing files and directories.

</div>

---

## Usage

```bash
sb [OPTIONS] [PATH]
```

### Examples

List entries in the current directory:

```bash
sb
```

List entries in the current directory, including hidden entries:

```bash
sb -a
```

List entries in the current directory, including hidden entries, and sort by size:

```bash
sb -a --sort=s
```

List entries in the current directory, recursively, including hidden entries and format as a tree-view with emoji icons:

```bash
sb -aRTe
```

## Command-line options

- [x] `-a`, `--all` 
    - do not ignore entries starting with .
- [x] `-c`, `--created`
    - display created at timestamp
- [x] `-d`, `--dirs`
    - show only directories 
- [x] `-e`, `--emoji`
    - prepend entries with emojis (📄, 📁, 🔗)
- [x] `-f`, `--files`
    - show only files
- [x] `-r`, `--reverse`
    - reverse sort order
- [ ] `-R`, `--recursive`
    - list directories recursively
- [x] `-s`, `--sort=WORD`
    - sort by WORD: name (`n`), ctime (`c`), mtime (`m`), size (`s`)
- [X] `-S`, `--size`
    - display size
- [ ] `-T`, `--tree`
    - format list as a tree-view
- [x] `-m`, `--modified`
    - display modified at timestamp
- [ ] `-p`
    - show permissions (`rwx`) for `[root][group][user]`


## License

[Shikibetsu (識別)](#shikibetsu-%E8%AD%98%E5%88%A5) is licensed under the MIT License. See [LICENSE](LICENSE) for more information.