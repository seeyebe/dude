# dude

**dude** is a secure, single-binary CLI utility that helps you **identify, preview, and safely remove orphaned packages** from your Arch Linux system. Featuring a colorized TUI and a safety-first workflow, dude is designed for maintainability and ease of use.

[![TUI Example](https://i.postimg.cc/C1ZjbBYp/image.png)](https://postimg.cc/5YVYVt6s)

---

## Installation

### AUR (Arch User Repository)

```bash
yay -S dude
```

### Build from Source

```bash
git clone https://github.com/seeyebe/dude
cd dude
cargo build --release
sudo cp target/release/dude /usr/local/bin/
```

---

## Usage

### Interactive Mode (TUI)

```bash
dude
```

or explicitly:

```bash
dude tui
```

* Use arrow keys to navigate
* Press `Space` to toggle selections
* Press `Enter` to confirm and remove selected packages

### List Orphans

```bash
dude list
```

### Prune Orphans

Dry run (default):

```bash
dude prune
```

Remove without confirmation:

```bash
dude prune --yes
```

Force dry run:

```bash
dude prune --dry
```

---

## Global Options

```bash
# Preserve packages matching pattern
dude --keep "^lib.*-dev$" prune

# Skip config file backup
dude --nosave prune --yes

# Quiet mode (e.g., for pacman hooks)
dude --hook list
```

---

## Configuration

Create a global or user-specific configuration file at:

* `/etc/dude.conf`
* `~/.config/dude/config`

Example:

```toml
whitelist = [
    "base-devel",
    "linux-headers"
]

[auto_prune]
threshold_mb = 100
days_since_last_run = 7
```

---

## Pacman Hook Integration

To receive orphan notifications after system updates, install the hook:

```bash
sudo cp hooks/dude.hook /usr/share/libalpm/hooks/
```

---

## License

dude is dual-licensed under the **Apache 2.0** or **MIT** license.

You may choose either license for your use case.
