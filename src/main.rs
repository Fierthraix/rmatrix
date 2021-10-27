extern crate pancurses;
extern crate r_matrix;
extern crate signal_hook;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use pancurses::*;
use signal_hook::consts::signal::{SIGINT, SIGQUIT, SIGTERM, SIGWINCH};
use signal_hook::flag;

use r_matrix::config::Config;
use r_matrix::Matrix;

fn main() {
    // Get command line args
    let mut config = Config::default();

    // Save the terminal state and start up ncurses
    let window = r_matrix::ncurses_init();

    // Create atomic bools that Unix signals can register.
    let exit_signal = Arc::new(AtomicBool::new(false));
    let resize_signal = Arc::new(AtomicBool::new(false));

    // Look for window-resize events.
    flag::register(SIGWINCH, Arc::clone(&resize_signal)).unwrap();

    // Look for exit-events.
    flag::register(SIGINT, Arc::clone(&exit_signal)).unwrap();
    flag::register(SIGTERM, Arc::clone(&exit_signal)).unwrap();
    flag::register(SIGQUIT, Arc::clone(&exit_signal)).unwrap();

    // Create the board
    let mut matrix: Matrix = Matrix::default();

    // Main event loop
    loop {
        // SIGWINCH: Make a new matrix for the new terminal size.
        if resize_signal.swap(false, Ordering::Relaxed) {
            r_matrix::resize_window();
            matrix = Matrix::default();
        }
        // Exit the program on exit signals (SIGINT, SIGTERM, SIGQUIT).
        if exit_signal.swap(false, Ordering::Relaxed) {
            r_matrix::finish();
        }

        // Handle a keypress.
        if let Some(Input::Character(c)) = window.getch() {
            config.handle_keypress(c)
        }

        // Update and redraw the board.
        matrix.arrange(&config);
        matrix.draw(&window, &config);
    }
}
