extern crate rand;
extern crate clap;
extern crate ncurses;

pub mod config;

use config::Config;

use ncurses::*;
use rand::{Rng, ThreadRng};

pub struct Matrix {
    m: Vec<Column>,
    cols: usize,
    lines: usize,
    rng: ThreadRng,
}

impl Matrix {
    /// Create a new matrix with the dimensions of the screen
    pub fn new() -> Self {
        // Get the screen dimensions
        let (lines, cols) = (COLS() as usize, LINES() as usize);

        // Create a seeded rng
        let mut rng = rand::thread_rng();

        // Create the matrix
        Matrix {
            m: (0..cols / 2)
                .map(|_| Column::new(lines, &mut rng))
                .collect(),
            cols: cols,
            lines: lines,
            rng: rng,
        }
    }
    pub fn cols(&self) -> usize {
        self.m.len()
    }
    pub fn lines(&self) -> usize {
        self.m[0].col.len()
    }
    pub fn arrange(&mut self, count: &mut usize, config: &Config) {
        let (lines, cols) = (self.m.len(), self.m[0].col.len());
        let mut rng = self.rng.clone(); // rng is Rc<RefCell<T>>, this avoids closure issues

        let (randnum, randmin, highnum) = if config.console || config.xwindow {
            (51, 166, 217)
        } else {
            (93, 33, 123)
        };

        self.m.iter_mut().for_each(
            |col| if !col.head && col.cchar == ' ' &&
                col.gap > 0
            {
                col.gap -= 1;
            } else if !col.head && col.cchar == ' ' {
                col.length = rng.gen::<usize>() % (lines - 3) + 3;
                col.col[0].val = rng.gen::<isize>() % randnum + randmin;

                if rng.gen::<usize>() % 2 == 1 {
                    col.col[0].bold = 2;
                }

                col.gap = rng.gen::<usize>() % lines + 1;
            },
        )
    }
}

struct Column {
    length: usize, // The length of the stream
    gap: usize, // The gap between streams
    update: usize, // Update speed
    head: bool,
    cchar: char,
    col: Vec<Block>, // The actual column
}

impl Column {
    /// Return a column keyed by a random number generator
    fn new(lines: usize, rand: &mut ThreadRng) -> Self {
        Column {
            length: rand.gen::<usize>() % (lines - 3) + 3,
            gap: rand.gen::<usize>() % lines + 1,
            update: rand.gen::<usize>() % 3 + 1,
            head: false,
            cchar: ' ',
            col: vec![Block::neg(); lines + 1],
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
