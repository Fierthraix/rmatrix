# rmatrix

Generates a 'Matrix'-like screen of falling characters in your terminal
[![rmatrix](rmatrix.gif)](https://asciinema.org/a/IjJyH88BeocsHvJpKJYqvmnuT)

The original [`cmatrix`](https://github.com/abishekvashok/cmatrix) was written in C, and crashes when you wildly resize the window.
The rust version is memory-safe, and doesn't crash so easily. Both versions have comparable performance, due to the underlying use of `ncurses`.

## Controls

| Key         | Control                                             |
| ----------- | --------------------------------------------------- |
| 1-9         | Speed the letters fall (1 is fastest, 9 is slowest) |
| Shift + 1-9 | Colour of the characters                            |
| r           | Rainbow mode                                        |
| p           | Pause the simulation                                |

## Installation

### cargo

`cargo install r-matrix`

### Arch Linux

`yay -S rmatrix`
