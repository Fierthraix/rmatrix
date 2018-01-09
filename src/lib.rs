extern crate rand;
extern crate pancurses;
extern crate term_size;

pub mod config;

use config::Config;

use pancurses::*;
use std::cell::RefCell;
use rand::{Rand, Rng, XorShiftRng};

thread_local!{
    static RNG: RefCell<XorShiftRng> = RefCell::new(rand::weak_rng());
}

fn gen<T: Rand>() -> T {
    RNG.with(|rng| (*rng).borrow_mut().gen::<T>())
}
fn rand_char() -> char {
    let (randnum, randmin) = (93, 33);
    (RNG.with(|rng| (*rng).borrow_mut().gen::<u8>() % randnum + randmin) as char)
}
fn rand_kana() -> String {
    RNG.with(|rng| String::from_utf16(&[(*rng).borrow_mut().gen_range(0xff62, 0xff9e)]).unwrap())
}
fn coin_flip() -> bool {
    RNG.with(|rng| (*rng).borrow_mut().gen())
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
        let (lines, cols) = get_term_size();

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
                                       col.new_rand_head(config);

                                       // Decrement length of stream
                                       col.length -= 1;

                                       // Reset number of spaces until next stream
                                       col.spaces = gen::<usize>() % lines + 1;
                                   } else if col.length != 0 {
                                       // Continue producing stream
                                       col.new_rand_char(config);
                                       col.length -= 1;
                                   } else {
                                       // Display spaces until next stream
                                       col.col[0] = Block::default();
                                       col.length = gen::<usize>() % (lines - 3) + 3;
                                   });
        if config.oldstyle {
            self.old_style_move_down();
        } else {
            self.move_down(config);
        }
    }
    fn move_down(&mut self, config: &Config) {
        self.m.iter_mut().for_each(|col| {
            // Reset for each column
            let mut in_stream = false;

            let mut last_was_white = false; // Keep track of white heads

            col.col.iter_mut().for_each(|block| {

                if !in_stream {
                    if !block.is_space() {
                        block.make_space();
                        in_stream = true; // We're now in a stream
                    }
                } else if block.is_space() {
                    // New rand char for head of stream
                    block.new_rand(config);
                    last_was_white = block.white();
                    in_stream = false;
                }
                // Swapped to "pass on" whiteness and prepare the variable for the next iteration
                block.set_white(last_was_white);
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
                    match gen::<usize>() % 6 {
                        0 => COLOR_GREEN,
                        1 => COLOR_BLUE,
                        2 => COLOR_WHITE,
                        3 => COLOR_YELLOW,
                        4 => COLOR_CYAN,
                        5 => COLOR_MAGENTA,
                        _ => unreachable!(),
                    }
                } else if self[i][j].white() {
                    COLOR_WHITE
                } else {
                    config.colour
                };
                // Draw the character
                window.attron(COLOR_PAIR(mcolour as u32));
                //window.addch(self[i][j].val as u32);
                self[i][j].draw(window);
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
            length: gen::<usize>() % (lines - 3) + 3,
            spaces: gen::<usize>() % lines + 1,
            col: vec![Block::default(); lines],
        }
    }
    fn head_is_empty(&self) -> bool {
        self.col[1].is_space()
    }
    fn new_rand_char(&mut self, config: &Config) {
        self.col[0] = Block::rand_block(config);
    }
    fn new_rand_head(&mut self, config: &Config) {
        self.col[0] = Block::rand_block(config);
        // 50/50 chance the head is white
        self.col[0].set_white(coin_flip());
    }
}

impl ops::Index<usize> for Column {
    type Output = Block;
    fn index(&self, i: usize) -> &Self::Output {
        &self.col[i]
    }
}

pub enum Block {
    Ascii((char, bool)),
    Kana((String, bool)),
    Empty,
}

impl Block {
    fn draw(&self, window: &Window) {
        match *self {
            Block::Ascii((val, _)) => {window.addch(val);},
            Block::Kana((ref val, _)) => {window.addstr(val);},
            Block::Empty => {},
        }
    }
    pub fn is_space(&self) -> bool {
        match *self {
            Block::Empty => true,
            _ => false
        }
    }
    pub fn new_rand(&mut self, config: &Config) {
        if config.kana {
            std::mem::swap(self, &mut Block::rand_block(config))
        } else {

        }
    }
    pub fn rand_block(config: &Config) -> Self {
        if config.kana {
            Block::Kana((rand_kana(), false))
        } else {
            Block::Ascii((rand_char(), false))
        }
    }
    pub fn rand_head(config: &Config) -> Self {
        if config.kana {
            Block::Kana((rand_kana(), coin_flip()))
        } else {
            Block::Ascii((rand_char(), coin_flip()))
        }
    }
    pub fn white(&self) -> bool {
        match *self {
            Block::Ascii((_, white)) | Block::Kana((_, white)) => white,
            Block::Empty => false
        }
    }
    pub fn set_white(&mut self, new_white: bool) {
        match *self {
            Block::Ascii((_, mut _white)) | Block::Kana((_, mut _white)) => _white = new_white,
            Block::Empty => {}
        }
    }
    pub fn make_space(&mut self) {
        std::mem::swap(self, &mut Block::default())
    }
}

impl Default for Block {
    fn default() -> Self {
        Block::Empty
    }
}

impl Clone for Block {
    fn clone(&self) -> Self {
        match *self {
            Block::Ascii((val, white)) => Block::Ascii((val, white)),
            Block::Kana((ref val, white)) => Block::Kana((val.clone(), white)),
            Block::Empty => Block::Empty,
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
