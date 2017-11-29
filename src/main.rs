extern crate ncurses;
extern crate rmatrix;

use std::env;
use rmatrix::*;
use ncurses::*;

fn main() {
    // Get command line args
    let args = get_args();

    let term = env::var("TERM").unwrap_or(String::from(""));

    // Force `$TERM` to be 'linux' if the user asked
    if args.is_present("force") && term.as_str() != "linux" {
        env::set_var("TERM", "linux");
    }

    initscr();
    savetty();
    nonl();
    cbreak();
    noecho();
    timeout(0);
    leaveok(stdscr(), true);
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    // TODO:
    // handle a SIGINT with finish()
    // handle a SIGWINCH with handle_sigwinch (terminal window size changed)

    // TODO: use console chars
    /*
       #ifdef HAVE_CONSOLECHARS
       if (console) {
       va_system("consolechars -d");
       }
       #elif defined(HAVE_SETFONT)
       if (console){
       va_system("setfont");
       }
       #endif
       */

    if has_colors() {
        start_color();
        if use_default_colors() != ERR {
            init_pair(COLOR_BLACK, -1, -1);
            init_pair(COLOR_GREEN, COLOR_GREEN, -1);
            init_pair(COLOR_WHITE, COLOR_WHITE, -1);
            init_pair(COLOR_RED, COLOR_RED, -1);
            init_pair(COLOR_CYAN, COLOR_CYAN, -1);
            init_pair(COLOR_MAGENTA, COLOR_MAGENTA, -1);
            init_pair(COLOR_BLUE, COLOR_BLUE, -1);
            init_pair(COLOR_YELLOW, COLOR_YELLOW, -1);
            println!("a");
        } else {
            init_pair(COLOR_BLACK, COLOR_BLACK, COLOR_BLACK);
            init_pair(COLOR_GREEN, COLOR_GREEN, COLOR_BLACK);
            init_pair(COLOR_WHITE, COLOR_WHITE, COLOR_BLACK);
            init_pair(COLOR_RED, COLOR_RED, COLOR_BLACK);
            init_pair(COLOR_CYAN, COLOR_CYAN, COLOR_BLACK);
            init_pair(COLOR_MAGENTA, COLOR_MAGENTA, COLOR_BLACK);
            init_pair(COLOR_BLUE, COLOR_BLUE, COLOR_BLACK);
            init_pair(COLOR_YELLOW, COLOR_YELLOW, COLOR_BLACK);
            println!("b");
        }

    }

    let (randnum, randmin, highnum) = if args.is_present("console") || args.is_present("xwindow") {
        (51, 166, 217)
    } else {
        (93, 33, 123)
    };

    //TODO: remove
    finish()
}
