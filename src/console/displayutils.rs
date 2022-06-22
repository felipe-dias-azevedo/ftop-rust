use std::io;
use std::io::Stdout;
use crossterm::{cursor, execute, terminal};

pub struct ConsoleDisplay {
    output: Stdout
}

pub struct TerminalSize {
    pub width: u16,
    pub height: u16
}

impl ConsoleDisplay {
    pub fn new() -> ConsoleDisplay {
        let stdout = io::stdout();

        ConsoleDisplay {
            output: stdout
        }
    }

    pub fn start(&mut self) {
        terminal::SetTitle("ftop");

        execute!(self.output, terminal::EnterAlternateScreen).unwrap();
        execute!(self.output, cursor::Hide).unwrap();
        execute!(self.output, terminal::Clear(terminal::ClearType::All)).unwrap();

        terminal::enable_raw_mode().unwrap();
    }

    pub fn shutdown(&mut self) {
        self.clear();

        execute!(self.output, terminal::LeaveAlternateScreen).unwrap();
        execute!(self.output, cursor::Show).unwrap();

        terminal::disable_raw_mode().unwrap();
    }

    pub fn clear(&mut self) {
        execute!(self.output, cursor::MoveTo(0, 0)).unwrap();
        execute!(self.output, terminal::Clear(terminal::ClearType::All)).unwrap();
    }

    pub fn get_terminal_size(&self) -> TerminalSize {
        let size = terminal::size().unwrap_or((0, 0));

        TerminalSize {
            width: size.0,
            height: size.1
        }
    }

    pub fn get_loadbar(&self, ammount: f32, margin: f32) -> String {
        let width_size = self.get_terminal_size().width as f32 * (1.0 - margin);

        let filled: usize = (width_size * (ammount / 100.0)) as usize;
        let empty: usize = width_size as usize - filled;

        format!("[{:#<filled$}{:.<empty$} {:.1} %]", "", "", ammount, filled=filled, empty=empty)
    }
}