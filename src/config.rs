use pancurses::*;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "rmatrix", about = "Shows a scrolling 'Matrix' like screen in linux")]
/// The struct for handling command line arguments
struct Opt {

    #[structopt(short = "b", parse(from_occurrences))]
    /// Bold characters on
    bold: isize,

    #[structopt(short = "l", long = "console")]
    /// Linux mode (use matrix console font)
    console: bool,

    #[structopt(short ="o", long = "oldstyle")]
    /// Use old-style scrolling
    oldstyle: bool,

    #[structopt(short = "s", long = "screensaver")]
    /// "Screensaver" mode, exits on first keystroke
    screensaver: bool,

    #[structopt(short = "x", long = "xwindow")]
    /// X window mode, use if your xterm is using mtx.pcf
    xwindow: bool,

    #[structopt(short = "u", long = "update", default_value = "4",
                parse(try_from_str = "validate_update"))]
    /// Screen update delay
    update: usize,

    #[structopt(short = "C", long = "colour", default_value = "green",
            raw(possible_values = r#"&[ "green", "red", "blue", "white", 
                "yellow", "cyan", "magenta", "black"]"#))]
    colour: String,

    #[structopt(short = "r", long = "rainbow")]
    /// Rainbow mode
    rainbow: bool,
}

fn validate_update(n: &str) -> Result<usize, &'static str> {
    if let Ok(n) = n.parse::<usize>() {
        if n <= 10 { return Ok(n) }
    }
    Err("must be a number between 1 and 10")
}

/// The global state object
pub struct Config {
    pub bold: isize,
    pub console: bool,
    pub oldstyle: bool,
    pub screensaver: bool,
    pub xwindow: bool,
    pub update: usize,
    pub colour: i16,
    pub rainbow: bool,
    pub pause: bool,
}

impl Config {
    /// Get the new config object based on command line arguments
    pub fn new() -> Self {
        let opt = Opt::from_args();

        let colour = match opt.colour.as_ref() {
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

        Config {
            bold: opt.bold,
            console: opt.console,
            oldstyle: opt.oldstyle,
            screensaver: opt.screensaver,
            xwindow: opt.xwindow,
            update: opt.update,
            rainbow: opt.rainbow,
            colour,
            pause: false,
        }
    }
    /// Update the config based on any keypresses
    pub fn handle_keypress(&mut self, keypress: char) {
        // Exit if in screensaver mode
        if self.screensaver {
            super::finish();
        }

        match keypress {
            'q' => {
                super::finish()
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
                self.update = keypress as usize - 48 // Sneaky way to avoid parsing
            }
            _ => {}
        }
    }
}

