pub mod list;
pub mod password;

use std::io;

use ratatui::{
    prelude::*,
    widgets::{block::Title, Block, Padding},
};

use crate::tui;
use clipboard_rs::{Clipboard, ClipboardContext};
use color_eyre::eyre::WrapErr;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    symbols::border,
    widgets::{block::Position, Borders},
};

use self::password::PasswordType;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Screens {
    Password(PasswordType),
    List,
}

#[derive(Debug)]
pub struct App {
    length: usize,
    password: String,
    list_state: list::Items<PasswordType>,
    screen: Screens,
    exit: bool,
}

impl Default for App {
    fn default() -> Self {
        let length = 8;
        let list_state = list::Items::<PasswordType>::default();

        let password_type = list_state
            .get_selected()
            .expect("Item should be selected");

        Self {
            length,
            password: password_type.generate(length),
            list_state,
            screen: Screens::List,
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
        match self.screen.clone() {
            Screens::Password(password_type) => match key_event.code {
                KeyCode::Char('q') => self.screen = Screens::List,
                KeyCode::Left | KeyCode::Char('h') => {
                    self.length = self
                        .length
                        .saturating_sub(1)
                        .max(*password_type.get_range().start());
                }
                KeyCode::Right | KeyCode::Char('l') => {
                    self.length = self
                        .length
                        .saturating_add(1)
                        .min(*password_type.get_range().end());
                }
                KeyCode::Down | KeyCode::Char('j' | 'k') | KeyCode::Up => {
                    if let Screens::Password(PasswordType::Random { ref mut state, .. }) =
                        self.screen
                    {
                        state.handle_key_event(key_event);
                    }
                }
                KeyCode::Enter | KeyCode::Char(' ') => {
                    if let Screens::Password(ref mut password_type) = self.screen {
                        password_type.handle_toggle();
                    }
                }
                KeyCode::Char('y' | 'c') => {
                    drop(
                        ClipboardContext::new().and_then(|ctx| ctx.set_text(self.password.clone())),
                    );
                }
                _ => {}
            },
            Screens::List => match key_event.code {
                KeyCode::Char('q') => self.exit(),
                KeyCode::Enter | KeyCode::Char(' ') => {
                    let selected = self
                        .list_state
                        .get_selected()
                        .cloned()
                        .unwrap_or_default();
                    self.screen = Screens::Password(selected);
                }
                KeyCode::Down | KeyCode::Char('j' | 'k') | KeyCode::Up => {
                    self.list_state
                        .handle_key_event(key_event);
                }
                _ => {}
            },
        };
    }

    fn update_password(&mut self, password_type: &PasswordType) {
        let range = password_type.get_range();
        self.length = self
            .length
            .clamp(*range.start(), *range.end());

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
    match app.screen.clone() {
        Screens::Password(password_type) => password::ui(frame, app, &password_type),
        Screens::List => list::ui(frame, &mut app.list_state),
    };
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
