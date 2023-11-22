<div align="center">

# Shikibetsu (識別)

[![Build Status](https://github.com/agnostk/shikibetsu/workflows/CI/badge.svg)](https://github.com/agnostk/shikibetsu/actions)
[![Code Coverage](https://agnostk.github.io/shikibetsu/coverage/badges/flat.svg)](https://agnostk.github.io/shikibetsu/coverage)

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

List entries in the current directory, recursively, including hidden entries and format with emoji icons:

```bash
sb -aRe
```

## Command-line options

- `-a`, `--all` 
    - do not ignore entries starting with .
- `-c`, `--created`
    - display created at timestamp
- `-d`, `--dirs`
    - show only directories 
- `-e`, `--emoji`
    - prepend entries with emojis (📄, 📁, 🔗)
- `-f`, `--files`
    - show only files
- `-r`, `--reverse`
    - reverse sort order
- `-R`, `--recursive`
    - list directories recursively
- `-s`, `--sort=WORD`
    - sort by WORD: name (`n`), ctime (`c`), mtime (`m`), size (`s`)
- `-S`, `--size`
    - display size
- `-m`, `--modified`
    - display modified at timestamp
- `-p`, `--perms`
    - show permissions (`rwx`) for `[root|group|user]`


## License

[Shikibetsu (識別)](#shikibetsu-%E8%AD%98%E5%88%A5) is licensed under the MIT License. See [LICENSE](LICENSE) for more information.