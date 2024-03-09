pub mod list;
pub mod password;

use std::io;

use ratatui::{
    prelude::*,
    widgets::{block::Title, Block, Padding},
};

use crate::tui;
use color_eyre::eyre::WrapErr;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    symbols::border,
    widgets::{block::Position, Borders},
};

use self::password::PasswordType;

#[derive(Debug)]
pub struct App {
    length: usize,
    password: String,
    list_state: list::Items<PasswordType>,
    exit: bool,
}

impl Default for App {
    fn default() -> Self {
        let length = 8;
        let list_state = list::Items::default();

        let password_type = list_state
            .get_selected()
            .expect("Item should be selected");

        Self {
            length,
            password: password_type.generate(length),
            list_state,
            exit: false,
        }
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut tui::Tui) -> color_eyre::Result<()> {
        while !self.exit {
            terminal.draw(|frame| ui(frame, self))?;
            self.handle_events()
                .wrap_err("handle events failed")?;
        }
        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        let selected = self
            .list_state
            .get_selected()
            .expect("item should be selected");
        let range = selected.get_range();

        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left | KeyCode::Char('h') => {
                self.length = self
                    .length
                    .saturating_sub(1)
                    .max(*range.start());
                self.update_password();
            }
            KeyCode::Right | KeyCode::Char('l') => {
                self.length = self
                    .length
                    .saturating_add(1)
                    .min(*range.end());
                self.update_password();
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.list_state.next();
                self.length = self
                    .length
                    .max(*range.start())
                    .min(*range.end());
                self.update_password();
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.list_state.previous();
                self.length = self
                    .length
                    .max(*range.start())
                    .min(*range.end());
                self.update_password();
            }
            _ => {}
        }
    }

    fn update_password(&mut self) {
        let password_type = self
            .list_state
            .get_selected()
            .expect("item should be selected");

        self.password = password_type.generate(self.length);
    }
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event);
            }
            _ => {}
        }

        Ok(())
    }
}

pub fn styled_block<'a>(title: Title<'a>, instructions: Title<'a>) -> Block<'a> {
    let block = Block::default()
        .title(title.alignment(Alignment::Center))
        .title(
            instructions
                .alignment(Alignment::Center)
                .position(Position::Bottom),
        )
        .borders(Borders::ALL)
        .border_set(border::THICK)
        .padding(Padding::horizontal(1));
    block
}

fn ui(frame: &mut Frame<'_>, app: &mut App) {
    let layout = Layout::horizontal([Constraint::Min(63), Constraint::Length(40)]);
    let [password_area, tabs_area] = layout.areas(frame.size());

    password::render(frame, password_area, app);
    list::render(frame, tabs_area, &mut app.list_state);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_increase_on_right_press() {
        let mut app = App::default();

        app.handle_key_event(KeyCode::Right.into());
        assert_eq!(app.length, 9);

        app.handle_key_event(KeyCode::Char('l').into());
        assert_eq!(app.length, 10);
    }

    #[test]
    fn should_decrease_on_left_press() {
        let mut app = App::default();

        app.handle_key_event(KeyCode::Left.into());
        assert_eq!(app.length, 7);

        app.handle_key_event(KeyCode::Char('h').into());
        assert_eq!(app.length, 6);
    }

    #[test]
    fn should_stop_at_max() {
        let mut app = App::default();

        for _ in 0..100 {
            app.handle_key_event(KeyCode::Right.into());
        }

        assert_eq!(app.length, 12);
    }

    #[test]
    fn should_stop_at_min() {
        let mut app = App::default();

        for _ in 0..100 {
            app.handle_key_event(KeyCode::Left.into());
        }

        assert_eq!(app.length, 3);
    }

    #[test]
    fn should_exit_on_q_press() {
        let mut app = App::default();
        app.handle_key_event(KeyCode::Char('q').into());
        assert!(app.exit);
    }
}
