#[macro_use]
extern crate crossbeam_channel as channel;
extern crate pancurses;
extern crate rmatrix;
extern crate signal_hook;

use std::os::raw::c_int;
use std::thread;

use pancurses::*;

use rmatrix::config::Config;
use rmatrix::Matrix;

fn notify(signals: &[c_int]) -> Result<channel::Receiver<c_int>, std::io::Error> {
    let (s, r) = channel::bounded(100);
    let signals = signal_hook::iterator::Signals::new(signals)?;
    thread::spawn(move || {
        for signal in signals.forever() {
            // TODO handle channel.SendError in result here
            s.send(signal);
        }
    });
    Ok(r)
}

fn main() {
    // Get command line args
    let mut config = Config::new();

    // Save the terminal state and start up ncurses
    let window = rmatrix::ncurses_init();

    // Register for UNIX signals
    // TODO remove expect here
    let signal = notify(&[signal_hook::SIGINT, signal_hook::SIGWINCH])
        .expect("Error setting up signal handling.");

    // Create the board
    let mut matrix = Matrix::new();

    // Main event loop
    loop {
        // Check for SIGINT or SIGWINCH
        select! {
            default => {},
            recv(signal) -> signal => {
                if let Ok(signal) = signal {
                    match signal {
                        // Terminate ncurses properly on SIGINT
                        signal_hook::SIGINT => rmatrix::finish(),
                        // Redraw the screen on SIGWINCH
                        signal_hook::SIGWINCH => {
                            rmatrix::resize_window();
                            matrix = Matrix::new();
                        },
                        _ => {}
                    }
                }
            },
        }

        // Handle a keypress
        if let Some(keypress) = window.getch() {
            if let Input::Character(c) = keypress {
                config.handle_keypress(c)
            }
        }

        // Updaate and redraw the board
        matrix.arrange(&config);
        matrix.draw(&window, &config);
    }
}
