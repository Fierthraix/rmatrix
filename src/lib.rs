extern crate rand;
extern crate clap;
extern crate ncurses;

use ncurses::*;

use rand::{Rng, ThreadRng};

pub struct Matrix(Vec<Column>);

impl Matrix {
    /// Create a new matrix with the dimensions of the screen
    pub fn new() -> Self {
        let (lines, cols) = (COLS() as usize, LINES() as usize);

        let mut rng = rand::thread_rng();

        // Create the matrix
        Matrix(
            (0..cols)
                .map(|i| if i % 2 == 0 {
                    Column::new(lines, &mut rng)
                } else {
                    Column::zero(lines)
                })
                .collect(),
        )
    }
}

struct Column {
    length: usize, // The length of the stream
    gap: usize, // The gap between streams
    update: usize, // Update speed
    col: Vec<Block>, // The actual column
}

impl Column {
    /// Return a column keyed by a random number generator
    fn new(lines: usize, rand: &mut ThreadRng) -> Self {
        let r1: usize = rand.gen();
        let r2: usize = rand.gen();
        let r3: usize = rand.gen();
        Column {
            length: r1 % (lines - 3) + 3,
            gap: r2 % lines + 1,
            update: r3 % 3 + 1,
            col: vec![Block::neg(); lines + 1],
        }
    }
    /// Return a zeroed column with blank values
    fn zero(lines: usize) -> Self {
        Column {
            length: 0,
            gap: 0,
            update: 0,
            col: vec![Block::new(); lines + 1],
        }
    }
}

#[derive(Clone)]
struct Block {
    //val: char,
    val: isize,
    bold: usize,
}

impl Block {
    fn new() -> Self {
        //Block { val: ' ', bold: 0 }
        Block { val: 0, bold: 0 }
    }
    fn neg() -> Self {
        Block { val: -1, bold: 0 }
    }
}

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
