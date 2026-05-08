# rmatrix
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

### cargo
`cargo install r-matrix`

### Arch Linux
Install the stable crates.io release from the AUR:

`yay -S rmatrix`

Or install the latest source build from git:

`yay -S rmatrix-git`

## Development

Install the local pre-commit hooks with:

`pre-commit install`

The hooks run `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test --all-targets --all-features`.
GitHub CI runs the same checks on Linux, macOS, and Windows.
