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

        self.m.iter_mut().for_each(|col| if col.head_is_empty() &&
            col.spaces != 0
        {
            // Decrement the spaces until the next stream starts
            col.spaces -= 1;
        } else if col.head_is_empty() && col.spaces == 0 {
            // Start the stream
            col.new_rand_char(&mut rng);

            // Decrement length of stream
            col.length -= 1;

            // Reset number of spaces until next stream
            col.spaces = rng.gen::<usize>() % lines + 1;
        } else if col.length != 0 {
            // Continue producing stream
            col.new_rand_char(&mut rng);
            col.length -= 1;
        } else {
            // Display spaces until next stream
            col.col[0].val = ' ';
            col.length = rng.gen::<usize>() % (lines - 3) + 3;
        });
        self.move_down();
    }
    fn move_down(&mut self) {
        // Iterate over all columns and swap spaces
        self.m.iter_mut().for_each(|col| {
            let mut tmp = Block::default(); // Blank space at head
            col.col.iter_mut().for_each(|mut block| {
                std::mem::swap(&mut tmp, &mut block);
            })
        });
    }
}

use std::fmt;

impl fmt::Debug for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut lines = vec![String::with_capacity(self.cols); self.lines];
        self.m.iter().for_each(|col| {
            col.col.iter().enumerate().for_each(|(i, val)| {
                lines[i].push(col.col[i].val)
            })
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

struct Column {
    length: usize, // The length of the stream
    spaces: usize, // The spaces between streams
    update: usize, // Update speed
    col: Vec<Block>, // The actual column
}

impl Column {
    /// Return a column keyed by a random number generator
    fn new(lines: usize, rand: &mut ThreadRng) -> Self {
        Column {
            length: rand.gen::<usize>() % (lines - 3) + 3,
            spaces: rand.gen::<usize>() % lines + 1,
            update: rand.gen::<usize>() % 3 + 1,
            col: vec![Block::default(); lines + 1],
        }
    }
    fn head_is_empty(&self) -> bool {
        self.col[1].val == ' '
    }
    fn new_rand_char(&mut self, rng: &mut ThreadRng) {
        //TODO: add a random character generator
        let (randnum, randmin) = (93, 33);
        self.col[0].val = (rng.gen::<u8>() % randnum + randmin) as char; // Random character

        // 50/50 chance the character is bold
        if rng.gen::<usize>() % 2 == 1 {
            self.col[1].bold = 2;
        }
    }
}

#[derive(Clone)]
struct Block {
    val: char,
    bold: usize,
}

impl Default for Block {
    fn default() -> Self {
        Block { val: ' ', bold: 0 }
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

#[test]
fn test_move_down_works() {
    fn block(c: char) -> Block {
        Block { val: c, bold: 0 }
    }
    let mut matrix = Matrix {
        rng: rand::thread_rng(),
        lines: 4,
        cols: 4,
        m: vec![
            Column {
                length: 2,
                spaces: 1,
                update: 2,
                col: vec![block('a'), block('b'), block('c'), block('d')],
            },
            Column {
                length: 2,
                spaces: 1,
                update: 2,
                col: vec![block('e'), block('f'), block('g'), block('h')],
            },
            Column {
                length: 2,
                spaces: 1,
                update: 2,
                col: vec![block('i'), block('j'), block('k'), block('l')],
            },
            Column {
                length: 2,
                spaces: 1,
                update: 2,
                col: vec![block('m'), block('n'), block('o'), block('p')],
            },
        ],
    };
    let matrix2 = Matrix {
        rng: rand::thread_rng(),
        lines: 4,
        cols: 4,
        m: vec![
            Column {
                length: 2,
                spaces: 1,
                update: 2,
                col: vec![block(' '), block('a'), block('b'), block('c')],
            },
            Column {
                length: 2,
                spaces: 1,
                update: 2,
                col: vec![block(' '), block('e'), block('f'), block('g')],
            },
            Column {
                length: 2,
                spaces: 1,
                update: 2,
                col: vec![block(' '), block('i'), block('j'), block('k')],
            },
            Column {
                length: 2,
                spaces: 1,
                update: 2,
                col: vec![block(' '), block('m'), block('n'), block('o')],
            },
        ],
    };

    matrix.move_down();
    assert_eq!(format!("{:?}", matrix), format!("{:?}", matrix2));
}
