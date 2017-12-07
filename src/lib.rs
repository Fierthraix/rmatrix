#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate ncurses;
extern crate term_size;

pub mod config;

use config::Config;

use ncurses::*;
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
                Some((width, height)) => (height + 1, width / 2),
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
    pub fn arrange(&mut self, config: &Config) {
        let lines = self.lines;

        self.m.iter_mut().for_each(|col| if col.head_is_empty() &&
            col.spaces != 0
        {
            // Decrement the spaces until the next stream starts
            col.spaces -= 1;
        } else if col.head_is_empty() && col.spaces == 0 {
            // Start the stream
            col.new_rand_char();

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
            col.col.iter_mut().for_each(|block| {
                if !in_stream {
                    if !block.is_space() {
                        block.val = ' ';
                        in_stream = true; // We're now in a stream
                    }
                } else {
                    if block.is_space() {
                        // New rand char for head of stream
                        block.val = (RNG.gen::<u8>() % 93 + 33) as char;
                        in_stream = false;
                    }
                }
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
    pub fn draw(&self, config: &Config) {
        for j in 1..self.lines {
            for i in 0..self.cols {
                mv(j as i32 - 1, 2 * i as i32); // Move the cursor
                if self[i][j].val == '\0' || self[i][j].bold == 2 {
                    if config.console || config.xwindow {
                        attron(A_ALTCHARSET as u32);
                    }
                    if config.bold == 1 {
                        //TODO: check this is 1 or 0
                        attron(A_BOLD as u32);
                    }
                    if self[i][j].val == '\0' {
                        if config.console || config.xwindow {
                            addch(183);
                        } else {
                            addch('&' as u32);
                        }
                    } else {
                        addch(self[i][j].val as u32);
                    }

                    attroff(COLOR_PAIR(COLOR_WHITE));
                    if config.bold == 1 {
                        attroff(A_BOLD as u32);
                    }
                    if config.console || config.xwindow {
                        attroff(A_ALTCHARSET as u32);
                    }
                } else {
                    let mcolor = if config.rainbow {
                        //TODO: Watch this for range problems (from the % 6)
                        match RNG.gen::<usize>() % 6 {
                            0 => COLOR_GREEN,
                            1 => COLOR_BLUE,
                            2 => COLOR_BLACK,
                            3 => COLOR_YELLOW,
                            4 => COLOR_CYAN,
                            5 => COLOR_MAGENTA,
                            _ => unreachable!(),
                        }
                    } else {
                        COLOR_GREEN
                    };
                    attron(COLOR_PAIR(mcolor));
                    if self[i][j].val == 1u8 as char {
                        if config.bold == 1 {
                            attron(A_BOLD as u32);
                        }
                        addch('|' as u32);
                        if config.bold == 1 {
                            attroff(A_BOLD as u32);
                        }
                    } else {
                        if config.console || config.xwindow {
                            attron(A_ALTCHARSET as u32);
                        }
                        if config.bold == 2 || (config.bold == 1 && self[i][j].val as u8 % 2 == 0) {
                            attron(A_BOLD as u32);
                        }
                        addch(self[i][j].val as u32);
                        if config.bold == 2 || (config.bold == 1 && self[i][j].val as u8 % 2 == 0) {
                            attroff(A_BOLD as u32);
                        }
                        if config.console || config.xwindow {
                            attroff(A_ALTCHARSET as u32);
                        }
                    }
                    attroff(COLOR_PAIR(mcolor));
                }
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

use std::fmt;

impl fmt::Debug for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut lines = vec![String::with_capacity(self.cols); self.lines];
        self.m.iter().for_each(|col| {
            col.col.iter().enumerate().for_each(
                |(i, val)| lines[i].push(val.val),
            )
        });
        let matrix = lines.into_iter().fold(
            String::with_capacity(self.lines * self.cols),
            |mut acc, m| {
                acc += &m;
                acc += "\n";
                acc
            },
        );
        write!(f, "{}", matrix)
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Matrix) -> bool {
        for (col1, col2) in self.m.iter().zip(other.m.iter()) {
            if col1 != col2 {
                return false;
            }
        }
        true
    }
}

pub struct Column {
    length: usize, // The length of the stream
    spaces: usize, // The spaces between streams
    update: usize, // Update speed
    col: Vec<Block>, // The actual column
}

impl Column {
    /// Return a column keyed by a random number generator
    fn new(lines: usize) -> Self {
        Column {
            length: RNG.gen::<usize>() % (lines - 3) + 3,
            spaces: RNG.gen::<usize>() % lines + 1,
            update: RNG.gen::<usize>() % 3 + 1,
            col: vec![Block::default(); lines],
        }
    }
    fn head_is_empty(&self) -> bool {
        self.col[1].val == ' '
    }
    fn new_rand_char(&mut self) {
        //TODO: add a random character generator
        let (randnum, randmin) = (93, 33);
        self.col[0].val = (RNG.gen::<u8>() % randnum + randmin) as char; // Random character

        // 50/50 chance the character is bold
        if RNG.gen::<usize>() % 2 == 1 {
            //TODO: find out why this is 1
            self.col[1].bold = 2;
        }
    }
}

impl ops::Index<usize> for Column {
    type Output = Block;
    fn index(&self, i: usize) -> &Self::Output {
        &self.col[i]
    }
}

impl PartialEq for Column {
    fn eq(&self, other: &Column) -> bool {
        for (blk1, blk2) in self.col.iter().zip(other.col.iter()) {
            if blk1 != blk2 {
                return false;
            }
        }
        true
    }
}

#[derive(Clone, PartialEq)]
pub struct Block {
    val: char,
    bold: usize,
}

impl Block {
    fn is_space(&self) -> bool {
        self.val == ' '
    }
}

impl Default for Block {
    fn default() -> Self {
        Block { val: ' ', bold: 0 }
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
}

/// Clean up ncurses stuff when we're ready to exit
pub fn finish() {
    curs_set(CURSOR_VISIBILITY::CURSOR_VISIBLE);
    clear();
    refresh();
    resetty();
    endwin();
    std::process::exit(0);
}

/// ncurses functions calls that set up the screen and set important variables
pub fn ncurses_init() {
    initscr();
    savetty();
    nonl();
    cbreak();
    noecho();
    timeout(0);
    leaveok(stdscr(), true);
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

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
}

pub fn resize_window() {
    //TODO: Find a way to do this without exiting ncurses
    endwin();
    initscr();
}
