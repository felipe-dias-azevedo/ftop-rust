use crossterm::style::Print;
use crossterm::{cursor, execute, terminal};
use std::io::Stdout;
use std::time::Duration;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, poll, read};
use crate::monitor::{Component, MonitorData};


pub fn start_message(system: &String) {
    println!("Starting monitoring on {}...", system);
}

pub fn list_components(components: Vec<MonitorData>) {
    for component in components {
        println!("---------- {} ----------", component.kind);
        for c in component.data {
            println!("{}: {}", c.id, c.name)
        }
    }
}

pub fn components_show(components: Vec<Component>) -> Vec<String> {
    components.into_iter().map(|c| format!("{}: {}", c.name, c.data)).collect::<Vec<String>>()
}

pub struct Display {
    stdout: Stdout,
    y: u16
}

impl Display {
    pub fn new(stdout: Stdout) -> Display {
        Display {
            stdout,
            y: 0
        }
    }

    pub fn start(&mut self) {
        terminal::SetTitle("ftop");

        execute!(self.stdout, terminal::EnterAlternateScreen).unwrap();
        execute!(self.stdout, cursor::Hide).unwrap();
        execute!(self.stdout, terminal::Clear(terminal::ClearType::All)).unwrap();

        terminal::enable_raw_mode().unwrap();
    }

    pub fn show(&mut self, message: String) {
        execute!(self.stdout, Print(message)).unwrap();
        self.jump_line();
    }

    pub fn stop(&mut self) {
        self.reset();

        execute!(self.stdout, terminal::LeaveAlternateScreen).unwrap();
        execute!(self.stdout, cursor::Show).unwrap();

        terminal::disable_raw_mode().unwrap();
    }

    fn jump_line(&mut self) {
        self.y += 1;
        execute!(self.stdout, cursor::MoveTo(0, self.y)).unwrap();
    }

    fn reset_cursor(&mut self) {
        self.y = 0;
        execute!(self.stdout, cursor::MoveTo(0, self.y)).unwrap();
    }

    pub fn reset(&mut self) {
        self.reset_cursor();
        execute!(self.stdout, terminal::Clear(terminal::ClearType::All)).unwrap();
    }

    pub fn read_stop_event(&self) -> bool {
        let has_event = poll(Duration::from_millis(50)).unwrap_or(false);

        if !has_event {
            return false;
        }

        match read() {
            Ok(v) => match v {
                Event::Key(KeyEvent {
                               code: KeyCode::Char('c'),
                               modifiers: KeyModifiers::CONTROL,
                               kind: KeyEventKind::Press,
                               state: KeyEventState::NONE,
                           }) => true,
                Event::Key(KeyEvent {
                               code: KeyCode::Char('q'),
                               modifiers: KeyModifiers::NONE,
                               kind: KeyEventKind::Press,
                               state: KeyEventState::NONE,
                           }) => true,
                Event::Key(KeyEvent {
                               code: KeyCode::Char('Q'),
                               modifiers: KeyModifiers::SHIFT,
                               kind: KeyEventKind::Press,
                               state: KeyEventState::NONE,
                           }) => true,
                Event::Key(KeyEvent {
                               code: KeyCode::Esc,
                               modifiers: KeyModifiers::NONE,
                               kind: KeyEventKind::Press,
                               state: KeyEventState::NONE,
                           }) => true,
                _ => false
            },
            _ => true,
        }
    }
}