# rmatrix

[![CI](https://github.com/Fierthraix/rmatrix/actions/workflows/ci.yml/badge.svg)](https://github.com/Fierthraix/rmatrix/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Fierthraix/rmatrix?display_name=tag)](https://github.com/Fierthraix/rmatrix/releases)
[![Crates.io](https://img.shields.io/crates/v/r-matrix.svg)](https://crates.io/crates/r-matrix)
[![Downloads](https://img.shields.io/crates/d/r-matrix.svg)](https://crates.io/crates/r-matrix)
[![Docs.rs](https://docs.rs/r-matrix/badge.svg)](https://docs.rs/r-matrix)
[![License](https://img.shields.io/crates/l/r-matrix.svg)](LICENSE)
[![AUR](https://img.shields.io/aur/version/rmatrix)](https://aur.archlinux.org/packages/rmatrix)
[![AUR bin](https://img.shields.io/aur/version/rmatrix-bin)](https://aur.archlinux.org/packages/rmatrix-bin)

Generates a 'Matrix'-like screen of falling characters in your terminal
[![rmatrix](rmatrix.gif)](https://asciinema.org/a/IjJyH88BeocsHvJpKJYqvmnuT)

The original [`cmatrix`](https://github.com/abishekvashok/cmatrix) was written in C, and crashes when you wildly resize the window.
The rust version is memory-safe, and doesn't crash so easily.
This version uses `crossterm` for cross-platform terminal support without needing `ncurses` installed.

## Controls
| Key | Control |
| -- | -- |
| 1-9 | Speed the letters fall (1 is fastest, 9 is slowest) |
| Shift + 1-9 | Colour of the characters |
| r | Rainbow mode |

## Installation

### Cargo

```bash
cargo install r-matrix
```

### Arch Linux / AUR

```bash
yay -S rmatrix
yay -S rmatrix-bin
```

### macOS / Homebrew

```zsh
brew install --cask Fierthraix/tap/rmatrix
```

### Windows / Scoop

```powershell
scoop bucket add fierthraix https://github.com/Fierthraix/scoop-bucket
scoop install rmatrix
```

### Nix

```bash
nix profile install github:Fierthraix/nur-packages#rmatrix
```

### Release Assets

```text
https://github.com/Fierthraix/rmatrix/releases/latest
```

## Development

Install the local pre-commit hooks with:

`pre-commit install`

The hooks run `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test --all-targets --all-features`.
GitHub CI runs the same checks on Linux, macOS, and Windows.
