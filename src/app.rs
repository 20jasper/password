use std::io;

use ratatui::{
    prelude::*,
    widgets::{block::Title, Block, Padding, Tabs},
};

use crate::{password, tui};
use color_eyre::eyre::WrapErr;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    symbols::border,
    widgets::{block::Position, Borders, Paragraph},
};

#[derive(Debug)]
pub struct App {
    length: usize,
    password: String,
    exit: bool,
}

impl Default for App {
    fn default() -> Self {
        let length = 8;
        let password = password::pin(length);

        Self {
            length,
            password,
            exit: false,
        }
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut tui::Tui) -> color_eyre::Result<()> {
        while !self.exit {
            terminal.draw(|frame| ui(frame, self))?;
            self.handle_events().wrap_err("handle events failed")?;
        }
        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left | KeyCode::Char('h') => {
                self.length = self.length.saturating_sub(1).max(6);
                self.update_password();
            }
            KeyCode::Right | KeyCode::Char('l') => {
                self.length = self.length.saturating_add(1).min(40);
                self.update_password();
            }
            _ => {}
        }
    }

    fn update_password(&mut self) {
        self.password = password::pin(self.length);
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

fn render_password_ui(frame: &mut Frame<'_>, area: Rect, app: &App) {
    let title = Title::from(" Password Generator ".bold());
    let instructions = Title::from(Line::from(vec![
        " Decrease Length ".into(),
        "<Left>/<H>".blue().bold(),
        " Increase Length ".into(),
        "<Right>/<L>".blue().bold(),
        " Quit ".into(),
        "<Q> ".blue().bold(),
    ]));
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

    let password_text = Paragraph::new(vec![
        Line::from(vec!["Length: ".into(), app.length.to_string().yellow()]),
        Line::from(vec!["Password: ".into(), app.password.clone().yellow()]),
    ])
    .block(block);

    frame.render_widget(password_text, area);
}

fn render_tabs_ui(frame: &mut Frame<'_>, area: Rect, app: &mut App) {
    let title = Title::from(" Password Types ".bold());
    let instructions = Title::from(Line::from(vec![
        " Next ".into(),
        "<Down>/<J>".blue().bold(),
        " Last ".into(),
        "<Up>/<K>".blue().bold(),
    ]));
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

    let password_text = Paragraph::new(vec![
        Line::from(vec!["Length: ".into(), app.length.to_string().yellow()]),
        Line::from(vec!["Password: ".into(), app.password.clone().yellow()]),
    ])
    .block(block);

    frame.render_widget(password_text, area);
}

fn ui(frame: &mut Frame<'_>, app: &mut App) {
    let layout = Layout::horizontal([Constraint::Min(20), Constraint::Length(40)]);
    let [password_area, tabs_area] = layout.areas(frame.size());

    render_password_ui(frame, password_area, app);
    render_tabs_ui(frame, tabs_area, app);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_key_event() {
        let mut app = App::default();
        app.handle_key_event(KeyCode::Right.into());
        assert_eq!(app.length, 1);

        app.handle_key_event(KeyCode::Left.into());
        assert_eq!(app.length, 0);

        app.handle_key_event(KeyCode::Left.into());
        assert_eq!(app.length, 0);

        let mut app = App::default();
        app.handle_key_event(KeyCode::Char('q').into());
        assert!(app.exit);
    }
}
