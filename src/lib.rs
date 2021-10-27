extern crate pancurses;
extern crate rand;
extern crate structopt;
extern crate term_size;

pub mod config;

use config::Config;

use pancurses::*;
use rand::distributions::{Distribution, Standard};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use std::cell::RefCell;

thread_local! {
    static RNG: RefCell<SmallRng> = RefCell::new(SmallRng::from_entropy());
}

fn gen<T>() -> T
where
    Standard: Distribution<T>,
{
    RNG.with(|rng| (*rng).borrow_mut().gen::<T>())
}
fn rand_char() -> char {
    let (randnum, randmin) = (93, 33);
    RNG.with(|rng| (*rng).borrow_mut().gen::<u8>() % randnum + randmin) as char
}
fn coin_flip() -> bool {
    RNG.with(|rng| (*rng).borrow_mut().gen())
}

pub struct Matrix {
    m: Vec<Column>,
    cols: usize,
    lines: usize,
}

impl Default for Matrix {
    /// Create a new matrix with the dimensions of the screen
    fn default() -> Self {
        // Get the screen dimensions
        let (lines, cols) = get_term_size();

        // Create the matrix
        Matrix {
            m: (0..cols).map(|_| Column::new(lines)).collect(),
            cols,
            lines,
        }
    }
}

impl Matrix {
    /// Make the next iteration of matrix
    pub fn arrange(&mut self, config: &Config) {
        let lines = self.lines;

        self.m.iter_mut().for_each(|col| {
            if col.head_is_empty() && col.spaces != 0 {
                // Decrement the spaces until the next stream starts
                col.spaces -= 1;
            } else if col.head_is_empty() && col.spaces == 0 {
                // Start a new stream
                col.new_rand_head(config);

                // Decrement length of stream
                col.length -= 1;

                // Reset number of spaces until next stream
                col.spaces = gen::<usize>() % lines + 1;
            } else if col.length != 0 {
                // Continue producing stream
                col.new_rand_char();
                col.length -= 1;
            } else {
                // Display spaces until next stream
                col.col[0].val = ' ';
                col.length = gen::<usize>() % (lines - 3) + 3;
            }
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
            let mut running_color = COLOR_CYAN;

            col.col.iter_mut().for_each(|block| {
                if !in_stream {
                    if !block.is_space() {
                        block.val = ' ';
                        in_stream = true; // We're now in a stream
                        running_color = block.color;
                    }
                } else if block.is_space() {
                    // New rand char for head of stream
                    block.val = rand_char();
                    block.white = last_was_white;
                    in_stream = false;
                }
                // Swapped to "pass on" whiteness and prepare the variable for the next iteration
                std::mem::swap(&mut last_was_white, &mut block.white);
                block.color = running_color;
            })
        })
    }
    fn old_style_move_down(&mut self) {
        // Iterate over all columns and swap spaces
        self.m.iter_mut().for_each(|col| {
            let mut tmp = Block::default(); // Blank space at head
            col.col.iter_mut().for_each(|block| {
                std::mem::swap(&mut tmp, block);
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
                let mcolour = if self[i][j].white {
                    COLOR_WHITE
                } else {
                    self[i][j].color
                };
                // Draw the character
                window.attron(COLOR_PAIR(mcolour as chtype));
                window.addch(self[i][j].val as chtype);
                window.attroff(COLOR_PAIR(mcolour as chtype));
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
    length: usize,   // The length of the stream
    spaces: usize,   // The spaces between streams
    col: Vec<Block>, // The actual column
}

impl Column {
    /// Return a column keyed by a random number generator
    fn new(lines: usize) -> Self {
        Column {
            length: gen::<usize>() % (lines - 3) + 3,
            spaces: gen::<usize>() % lines + 1,
            col: vec![Block::default(); lines],
        }
    }
    fn head_is_empty(&self) -> bool {
        self.col[1].val == ' '
    }
    fn new_rand_char(&mut self) {
        self.col[0].val = rand_char();
        self.col[0].color = self.col[1].color;
    }
    fn new_rand_head(&mut self, config: &Config) {
        self.col[0].val = rand_char();
        self.col[0].color = if config.rainbow {
            match gen::<usize>() % 6 {
                0 => COLOR_GREEN,
                1 => COLOR_BLUE,
                2 => COLOR_WHITE,
                3 => COLOR_YELLOW,
                4 => COLOR_CYAN,
                5 => COLOR_MAGENTA,
                _ => unreachable!(),
            }
        } else {
            config.colour
        };
        // 50/50 chance the head is white
        self.col[0].white = coin_flip();
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
    color: i16,
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
            color: COLOR_RED,
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

fn get_term_size() -> (usize, usize) {
    match term_size::dimensions() {
        Some((mut width, mut height)) => {
            // Minimum size for terminal
            if width < 10 {
                width = 10
            }
            if height < 10 {
                height = 10
            }
            if width % 2 != 0 {
                // Makes odd-columned screens print on the rightmost edge
                (height + 1, (width / 2) + 1)
            } else {
                (height + 1, width / 2)
            }
        }
        None => (10, 10),
    }
}

pub fn resize_window() {
    //TODO: Find a way to do this without exiting ncurses
    endwin();
    initscr();
}
