# omablue-bluetooth-tui

Modern TUI Bluetooth manager for OddOs/secureblue with dynamic theme integration.

## Features

- **Modern TUI interface** — Built with Ratatui, responsive and keyboard-driven
- **DBus-native BlueZ integration** — Zero C dependencies (via zbus), eliminates bash workarounds
- **omablue theme integration** — Reads and live-reloads `colors.toml` for consistent visual style
- **Device discovery** — Real-time device scanning via ObjectManager
- **Connection management** — Connect, disconnect, pair, and trust devices
- **Pairing agent** — Interactive confirmation dialog for secure pairing
- **Desktop notifications** — Integration with Dunst via notify-send
- **Memory-safe** — Pure Rust, no buffer overflows, no segfaults
- **secureblue-compliant** — No setuid, no root, minimal attack surface, XDG paths

## Installation

### Prerequisites

- Rust 1.70+ (for building)
- BlueZ (system)
- Fedora Silverblue/secureblue

### Build

```bash
git clone https://github.com/odd-git/omablue-bluetooth-tui.git
cd omablue-bluetooth-tui
cargo build --release
```

The compiled binary is at `target/release/omablue-bluetooth-tui` (~5-8 MB).

### Install to OddOs

Copy the binary to the omablue bin directory:

```bash
cp target/release/omablue-bluetooth-tui ~/.local/share/omablue/bin/
chmod +x ~/.local/share/omablue/bin/omablue-bluetooth-tui
```

Or integrate into the OddOs image build:

```bash
# In OddOs repo:
cp target/release/omablue-bluetooth-tui \
  files/system/home/mino/.local/share/omablue/bin/
```

## Usage

### Launch from TUI launcher

```bash
omablue-launch-tui "TUI.float" "omablue-bluetooth-tui" "120x35"
```

Or directly:

```bash
omablue-bluetooth-tui
```

### Keybindings

| Key | Action |
|-----|--------|
| `↑`/`↓` or `k`/`j` | Navigate device list |
| `Enter` | Connect/disconnect selected device |
| `d` | Force disconnect |
| `s` | Start/stop discovery scan |
| `p` | Enter pairing mode |
| `t` | Toggle adapter power (Bluetooth on/off) |
| `T` | Set device as trusted (auto-connect) |
| `?` | Show help |
| `q` or `Esc` | Quit |

## Architecture

### Event Loop (tokio)

- **Bluetooth scanner**: ObjectManager stream for device discovery
- **Theme watcher**: inotify on `~/.config/omablue/current/theme/colors.toml`
- **Input handler**: Crossterm event polling
- **Main loop**: MVU pattern (Model-View-Update)

### BlueZ via zbus

All Bluetooth operations go through DBus proxies (Adapter1, Device1) with no subprocess spawning.

### Theme System

Reads TOML colors from `~/.config/omablue/current/theme/colors.toml` and dynamically applies them to the TUI. Changes are reflected in real-time without restart.

## Dependencies

| Crate | Purpose |
|-------|---------|
| `ratatui` | TUI rendering |
| `crossterm` | Terminal raw mode, keyboard events |
| `tokio` | Async runtime |
| `zbus` | DBus client (zero libdbus) |
| `toml` / `serde` | Theme TOML parsing |
| `notify` | File watcher (inotify) |
| `notify-rust` | Desktop notifications |
| `dirs` | XDG paths |

**Zero C dependencies** — no `libdbus`, `libbluetooth`, or `libssl` required.

## Binary Size

- Release build: **4-8 MB** with `lto=true` and `strip=true`
- Minimal footprint suitable for rpm-ostree layering

## Testing

```bash
cargo test
cargo clippy -- -D warnings
```

Verify DBus connectivity:

```bash
busctl --user call org.bluez /org/bluez/hci0 org.bluez.Adapter1 GetDiscoveryFilters
```

## License

MIT

## Author

odd-git <oddai@tuta.io>
