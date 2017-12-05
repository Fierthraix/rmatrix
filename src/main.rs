extern crate ncurses;
extern crate rmatrix;

use std::env;
use ncurses::*;
use rmatrix::Matrix;
use rmatrix::config::Config;

fn main() {
    // Get command line args
    let mut config = Config::new();

    let term = env::var("TERM").unwrap_or(String::from(""));

    // Force `$TERM` to be 'linux' if the user asked
    if config.force && term.as_str() != "linux" {
        env::set_var("TERM", "linux");
    }

    rmatrix::ncurses_init();

    let mut matrix = Matrix::new();

    let mut count = 0;
    // The main event loop
    loop {
        count += 1;
        if count > 4 {
            count = 1;
        }

        // Handle a keypress
        let keypress = wgetch(stdscr());
        if keypress != ERR {
            // Exit if you're in screensaver mode
            if config.screensaver {
                rmatrix::finish();
            }

            // Update any config options based on user input
            config.update_from_keypress(keypress as u8 as char);
            // Check any config changes mean you need to exit the loop
            if config.should_break {
                break;
            }
        }

        matrix.arrange(&mut count, &config);
    }

    // Reset the old `$TERM` value if you changed it
    if config.force && term.as_str() != "" {
        env::set_var("TERM", term.as_str());
    }
    rmatrix::finish()
}
