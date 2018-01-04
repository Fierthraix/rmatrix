#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate pancurses;
extern crate term_size;

pub mod config;

use config::Config;

use pancurses::*;
use std::sync::Mutex;
use rand::{Rand, Rng, XorShiftRng};

lazy_static!{
    static ref RNG: MRng = MRng::new();
}

pub struct Matrix {
    m: Vec<Column>,
    cols: usize,
    lines: usize,
}

impl Matrix {
    /// Create a new matrix with the dimensions of the screen
    pub fn new() -> Self {
        // Get the screen dimensions
        let (lines, cols) = {
            match term_size::dimensions() {
                Some((width, height)) => {
                    if width % 2 != 0 {
                        // Makes odd-columned screens print on the rightmost edge
                        (height + 1, (width / 2) + 1)
                    } else {
                        (height + 1, width / 2)
                    }
                }
                None => (10, 10),
            }
        };

        // Create the matrix
        Matrix {
            m: (0..cols).map(|_| Column::new(lines)).collect(),
            cols: cols,
            lines: lines,
        }
    }
    /// Make the next iteration of matrix
    pub fn arrange(&mut self, config: &Config) {
        let lines = self.lines;

        self.m.iter_mut().for_each(|col| if col.head_is_empty() &&
            col.spaces != 0
        {
            // Decrement the spaces until the next stream starts
            col.spaces -= 1;
        } else if col.head_is_empty() && col.spaces == 0 {
            // Start a new stream
            col.new_rand_head();

            // Decrement length of stream
            col.length -= 1;

            // Reset number of spaces until next stream
            col.spaces = RNG.gen::<usize>() % lines + 1;
        } else if col.length != 0 {
            // Continue producing stream
            col.new_rand_char();
            col.length -= 1;
        } else {
            // Display spaces until next stream
            col.col[0].val = ' ';
            col.length = RNG.gen::<usize>() % (lines - 3) + 3;
        });
        if config.oldstyle {
            self.old_style_move_down();
        } else {
            self.move_down();
        }
    }
    fn move_down(&mut self) {
        self.m.iter_mut().for_each(|col| {
            // Reset for each column
            let mut in_stream = false;

            let mut last_was_white = false; // Keep track of white heads

            col.col.iter_mut().for_each(|block| {

                if !in_stream {
                    if !block.is_space() {
                        block.val = ' ';
                        in_stream = true; // We're now in a stream
                    }
                } else if block.is_space() {
                    // New rand char for head of stream
                    block.val = RNG.rand_char();
                    block.white = last_was_white;
                    in_stream = false;
                }
                // Swapped to "pass on" whiteness and prepare the variable for the next iteration
                std::mem::swap(&mut last_was_white, &mut block.white);
            })
        })
    }
    fn old_style_move_down(&mut self) {
        // Iterate over all columns and swap spaces
        self.m.iter_mut().for_each(|col| {
            let mut tmp = Block::default(); // Blank space at head
            col.col.iter_mut().for_each(|mut block| {
                std::mem::swap(&mut tmp, &mut block);
            })
        });
    }
    /// Draw the matrix on the screen
    pub fn draw(&self, window: &Window, config: &Config) {
        //TODO: Refactor this to cache mcolour and reduce calls to `attron`/`attroff`
        //TODO: Use an iterator or something nicer
        for j in 1..self.lines {
            for i in 0..self.cols {
                window.mv(j as i32 - 1, 2 * i as i32); // Move the cursor
                // Pick the colour we need
                let mcolour = if config.rainbow {
                    match RNG.gen::<usize>() % 6 {
                        0 => COLOR_GREEN,
                        1 => COLOR_BLUE,
                        2 => COLOR_WHITE,
                        3 => COLOR_YELLOW,
                        4 => COLOR_CYAN,
                        5 => COLOR_MAGENTA,
                        _ => unreachable!(),
                    }
                } else if self[i][j].white {
                    COLOR_WHITE
                } else {
                    config.colour
                };
                // Draw the character
                window.attron(COLOR_PAIR(mcolour as u32));
                window.addch(self[i][j].val as u32);
                window.attroff(COLOR_PAIR(mcolour as u32));
            }
        }
        napms(config.update as i32 * 10);
    }
}

use std::ops;

impl ops::Index<usize> for Matrix {
    type Output = Column;
    fn index(&self, i: usize) -> &Self::Output {
        &self.m[i]
    }
}

pub struct Column {
    length: usize, // The length of the stream
    spaces: usize, // The spaces between streams
    col: Vec<Block>, // The actual column
}

impl Column {
    /// Return a column keyed by a random number generator
    fn new(lines: usize) -> Self {
        Column {
            length: RNG.gen::<usize>() % (lines - 3) + 3,
            spaces: RNG.gen::<usize>() % lines + 1,
            col: vec![Block::default(); lines],
        }
    }
    fn head_is_empty(&self) -> bool {
        self.col[1].val == ' '
    }
    fn new_rand_char(&mut self) {
        self.col[0].val = RNG.rand_char();
    }
    fn new_rand_head(&mut self) {
        self.col[0].val = RNG.rand_char();
        // 50/50 chance the head is white
        self.col[0].white = RNG.coin_flip();
    }
}

impl ops::Index<usize> for Column {
    type Output = Block;
    fn index(&self, i: usize) -> &Self::Output {
        &self.col[i]
    }
}

#[derive(Clone)]
pub struct Block {
    val: char,
    white: bool,
}

impl Block {
    fn is_space(&self) -> bool {
        self.val == ' '
    }
}

impl Default for Block {
    fn default() -> Self {
        Block {
            val: ' ',
            white: false,
        }
    }
}

struct MRng(Mutex<XorShiftRng>);

impl MRng {
    fn new() -> Self {
        MRng(Mutex::new(rand::weak_rng()))
    }
    fn gen<T: Rand>(&self) -> T {
        match self.0.lock() {
            Ok(mut rng) => rng.gen::<T>(),
            Err(e) => panic!("{}", e),
        }
    }
    fn rand_char(&self) -> char {
        let (randnum, randmin) = (93, 33);
        (self.gen::<u8>() % randnum + randmin) as char
    }
    fn coin_flip(&self) -> bool {
        match self.0.lock() {
            Ok(mut rng) => rng.gen_weighted_bool(2),
            Err(e) => panic!("{}", e),
        }
    }
}

/// Clean up ncurses stuff when we're ready to exit
pub fn finish() {
    curs_set(1);
    endwin();
    std::process::exit(0);
}

/// ncurses functions calls that set up the screen and set important variables
pub fn ncurses_init() -> Window {
    let window = initscr();
    window.nodelay(true);
    window.refresh();

    noecho();
    nonl();
    cbreak();
    curs_set(0);

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
        } else {
            init_pair(COLOR_BLACK, COLOR_BLACK, COLOR_BLACK);
            init_pair(COLOR_GREEN, COLOR_GREEN, COLOR_BLACK);
            init_pair(COLOR_WHITE, COLOR_WHITE, COLOR_BLACK);
            init_pair(COLOR_RED, COLOR_RED, COLOR_BLACK);
            init_pair(COLOR_CYAN, COLOR_CYAN, COLOR_BLACK);
            init_pair(COLOR_MAGENTA, COLOR_MAGENTA, COLOR_BLACK);
            init_pair(COLOR_BLUE, COLOR_BLUE, COLOR_BLACK);
            init_pair(COLOR_YELLOW, COLOR_YELLOW, COLOR_BLACK);
        }
    }

    window
}

pub fn resize_window() {
    //TODO: Find a way to do this without exiting ncurses
    endwin();
    initscr();
}
