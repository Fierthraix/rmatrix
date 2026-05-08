extern crate crossterm;
extern crate r_matrix;
#[cfg(unix)]
extern crate signal_hook;

use std::io;
#[cfg(unix)]
use std::sync::Arc;
#[cfg(unix)]
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use crossterm::event::{Event, KeyCode, KeyModifiers};
#[cfg(unix)]
use signal_hook::consts::signal::{SIGINT, SIGQUIT, SIGTERM, SIGWINCH};
#[cfg(unix)]
use signal_hook::flag;

use r_matrix::Matrix;
use r_matrix::Terminal;
use r_matrix::config::Config;

#[cfg(unix)]
struct SignalFlags {
    exit_signal: Arc<AtomicBool>,
    resize_signal: Arc<AtomicBool>,
}

#[cfg(unix)]
impl SignalFlags {
    fn new() -> io::Result<Self> {
        // Create atomic bools that Unix signals can register.
        let exit_signal = Arc::new(AtomicBool::new(false));
        let resize_signal = Arc::new(AtomicBool::new(false));

        // Look for window-resize events.
        flag::register(SIGWINCH, Arc::clone(&resize_signal))?;

        // Look for exit-events.
        flag::register(SIGINT, Arc::clone(&exit_signal))?;
        flag::register(SIGTERM, Arc::clone(&exit_signal))?;
        flag::register(SIGQUIT, Arc::clone(&exit_signal))?;

        Ok(SignalFlags {
            exit_signal,
            resize_signal,
        })
    }

    fn should_exit(&self) -> bool {
        self.exit_signal.swap(false, Ordering::Relaxed)
    }

    fn should_resize(&self) -> bool {
        self.resize_signal.swap(false, Ordering::Relaxed)
    }
}

#[cfg(not(unix))]
struct SignalFlags;

#[cfg(not(unix))]
impl SignalFlags {
    fn new() -> io::Result<Self> {
        Ok(SignalFlags)
    }

    fn should_exit(&self) -> bool {
        false
    }

    fn should_resize(&self) -> bool {
        false
    }
}

fn main() -> io::Result<()> {
    // Get command line args
    let mut config = Config::default();

    // Save the terminal state and start up the terminal
    let mut terminal = Terminal::new()?;

    let signals = SignalFlags::new()?;

    // Create the board
    let mut matrix: Matrix = Matrix::default();

    // Main event loop
    loop {
        // SIGWINCH: Make a new matrix for the new terminal size.
        if signals.should_resize() {
            terminal.resize_window()?;
            matrix = Matrix::default();
        }
        // Exit the program on exit signals (SIGINT, SIGTERM, SIGQUIT).
        if signals.should_exit() {
            break;
        }

        // Handle a terminal event.
        if let Some(event) = terminal.get_event(Duration::from_millis(0))? {
            match event {
                Event::Resize(_, _) => {
                    terminal.resize_window()?;
                    matrix = Matrix::default();
                }
                Event::Key(key) => {
                    if config.screensaver
                        || key.modifiers.contains(KeyModifiers::CONTROL)
                            && matches!(key.code, KeyCode::Char('c' | 'C'))
                    {
                        break;
                    }

                    if let KeyCode::Char(c) = key.code
                        && config.handle_keypress(c)
                    {
                        break;
                    }
                }
                _ => {}
            }
        }

        // Update and redraw the board.
        matrix.arrange(&config);
        matrix.draw(&mut terminal, &config)?;
    }

    terminal.finish()
}
