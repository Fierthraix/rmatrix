extern crate clap;

use ncurses::*;
use self::clap::{Arg, App, ArgMatches};

pub struct Config {
    pub bold: isize,
    pub force: bool,
    pub console: bool,
    pub oldstyle: bool,
    pub screensaver: bool,
    pub xwindow: bool,
    pub update: usize,
    pub colour: i16,
    pub rainbow: bool,
    pub pause: bool,
    pub should_break: bool,
}

impl Config {
    /// Get the new config object based on command line arguments
    pub fn new() -> Self {
        let args = get_args();

        let colour = match args.value_of("colour").unwrap() {
            "green" => COLOR_GREEN,
            "red" => COLOR_RED,
            "blue" => COLOR_BLUE,
            "white" => COLOR_WHITE,
            "yellow" => COLOR_YELLOW,
            "cyan" => COLOR_CYAN,
            "magenta" => COLOR_MAGENTA,
            "black" => COLOR_BLACK,
            _ => unreachable!(),
        };

        let bold = if args.is_present("nobold") {
            0
        } else if args.is_present("B") {
            2
        } else if args.is_present("b") {
            1
        } else {
            0
        };

        Config {
            bold: bold,
            force: args.is_present("force"),
            console: args.is_present("console"),
            oldstyle: args.is_present("oldstyle"),
            screensaver: args.is_present("screensaver"),
            xwindow: args.is_present("xwindow"),
            update: args.value_of("update").unwrap().parse::<usize>().unwrap(),
            colour: colour,
            rainbow: args.is_present("rainbow"),
            pause: false,
            should_break: false,
        }
    }
    pub fn update_from_keypress(&mut self, keypress: char) {
        match keypress {
            'q' => {
                self.should_break = true;
            }
            'b' => self.bold = 1,
            'B' => self.bold = 2,
            'n' => self.bold = 0,
            '!' => {
                self.colour = COLOR_RED;
                self.rainbow = false;
            }
            '@' => {
                self.colour = COLOR_GREEN;
                self.rainbow = false;
            }
            '#' => {
                self.colour = COLOR_YELLOW;
                self.rainbow = false;
            }
            '$' => {
                self.colour = COLOR_BLUE;
                self.rainbow = false;
            }
            '%' => {
                self.colour = COLOR_MAGENTA;
                self.rainbow = false;
            }
            'r' => {
                self.rainbow = true;
            }
            '^' => {
                self.colour = COLOR_CYAN;
                self.rainbow = false;
            }
            '&' => {
                self.colour = COLOR_WHITE;
                self.rainbow = false;
            }
            'p' | 'P' => self.pause = !self.pause,
            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => {
                self.update = keypress as usize - 48
            }
            _ => {}
        }
    }
}

/// Get and parse the command line arguments with clap
fn get_args() -> ArgMatches<'static> {
    App::new("rmatrix")
        .version("0.0.1")
        .about("Shows a scrolling 'Matrix' like screen in linux")
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
