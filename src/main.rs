extern crate ncurses;
extern crate rmatrix;

mod config;

use std::env;
use rmatrix::*;
use ncurses::*;

use config::Config;

fn main() {
    // Get command line args
    let config = Config::new();

    let term = env::var("TERM").unwrap_or(String::from(""));

    // Force `$TERM` to be 'linux' if the user asked
    if config.force && term.as_str() != "linux" {
        env::set_var("TERM", "linux");
    }

    ncurses_init();

    let (randnum, randmin, highnum) = if config.console || config.xwindow {
        (51, 166, 217)
    } else {
        (93, 33, 123)
    };

    let matrix = Matrix::new();

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
                finish();
            }
            match keypress as u8 as char {
                'q' => {
                    finish();
                    break;
                }
                _ => {}
            }
        }
    }

    // Set the old `$TERM` value if you changed it
    if config.force && term.as_str() != "" {
        env::set_var("TERM", term.as_str());
    }
    finish()
}
