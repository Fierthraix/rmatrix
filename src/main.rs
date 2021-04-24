#[macro_use]
extern crate chan;
extern crate chan_signal;
extern crate pancurses;
extern crate r_matrix;

use chan_signal::Signal;
use pancurses::*;

use r_matrix::config::Config;
use r_matrix::Matrix;

fn main() {
    // Get command line args
    let mut config = Config::default();

    // Save the terminal state and start up ncurses
    let window = r_matrix::ncurses_init();

    // Register for UNIX signals
    let signal = chan_signal::notify(&[Signal::INT, Signal::WINCH]);

    // Create the board
    let mut matrix = Matrix::new();

    // Main event loop
    loop {
        // Check for SIGINT or SIGWINCH
        chan_select! {
            default => {},
            signal.recv() -> signal => {
                if let Some(signal) = signal {
                    match signal {
                        // Terminate ncurses properly on SIGINT
                        Signal::INT => r_matrix::finish(),
                        // Redraw the screen on SIGWINCH
                        Signal::WINCH => {
                            r_matrix::resize_window();
                            matrix = Matrix::new();
                        },
                        _ => {}
                    }
                }
            },
        }

        // Handle a keypress
        if let Some(Input::Character(c)) = window.getch() {
            config.handle_keypress(c)
        }

        // Updaate and redraw the board
        matrix.arrange(&config);
        matrix.draw(&window, &config);
    }
}
