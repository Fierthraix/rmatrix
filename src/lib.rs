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
