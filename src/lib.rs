extern crate clap;
extern crate ncurses;

use ncurses::*;
use clap::{Arg, App, ArgMatches};

/// Get and parse the command line arguments with clap
pub fn get_args() -> ArgMatches<'static> {
    App::new("rmatrix")
        .version("0.0.1")
        .about("Shows a scrolling 'Matrix' like screen in linux")
        .arg(Arg::with_name("async").short("a").help(
            "Asynchronous scroll",
        ))
        .arg(Arg::with_name("b").short("b").group("bold").help(
            "Bold characters on",
        ))
        .arg(Arg::with_name("B").short("B").help(
            "All bold characters (overrides -b)",
        ))
        .arg(Arg::with_name("force").short("f").help(
            "Force the linux $TERM type to be on",
        ))
        .arg(Arg::with_name("console").short("l").help(
            "Linux mode (use matrix console font)",
        ))
        .arg(Arg::with_name("oldstyle").short("o").help(
            "Use old-style scrolling",
        ))
        .arg(Arg::with_name("nobold").short("n").help(
            "No bold characters (overrides -b and -B, default)",
        ))
        .arg(Arg::with_name("screensaver").short("s").help(
            "\"Screensaver\" mode, exits on first keystroke",
        ))
        .arg(Arg::with_name("xwindow").short("x").help(
            "X window mode, use if your xterm is using mtx.pcf",
        ))
        .arg(
            Arg::with_name("update")
                .short("u")
                .value_name("delay")
                .default_value("4")
                .validator(|n: String| match n.parse::<u8>() {
                    Ok(n) => {
                        if n > 10 {
                            Err(String::from("the number must be between 0 and 10"))
                        } else {
                            Ok(())
                        }
                    }
                    Err(_) => Err(String::from("not a valid number between 0 and 10")),
                })
                .hide_default_value(true)
                .help("delay Screen update delay"),
        )
        .arg(
            Arg::with_name("colour")
                .short("C")
                .value_name("color")
                .default_value("green")
                .possible_values(
                    &[
                        "green",
                        "red",
                        "blue",
                        "white",
                        "yellow",
                        "cyan",
                        "magenta",
                        "black",
                    ],
                )
                .help("Use this colour for matrix"),
        )
        .arg(Arg::with_name("rainbow").short("r").help("Rainbow mode"))
        .get_matches()
}

/// Clean up ncurses stuff when we're ready to exit
pub fn finish() {
    curs_set(CURSOR_VISIBILITY::CURSOR_VISIBLE);
    clear();
    refresh();
    resetty();
    endwin();
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
}
