use std::io;

use ratatui::{
    prelude::*,
    widgets::{block::Title, Block, Tabs},
};

use crate::tui;
use color_eyre::eyre::WrapErr;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    symbols::border,
    widgets::{block::Position, Borders, Paragraph},
};

#[derive(Debug, Default)]
pub struct App {
    length: usize,
    password: String,
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut tui::Tui) -> color_eyre::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events().wrap_err("handle events failed")?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame<'_>) {
        frame.render_widget(self, frame.size());
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => {
                self.length = self.length.saturating_sub(1);
                self.update_password();
            }
            KeyCode::Right => {
                self.length = self.length.saturating_add(1).min(40);
                self.update_password();
            }
            _ => {}
        }
    }

    fn update_password(&mut self) {
        self.password = "*".repeat(self.length);
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

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Counter App Tutorial ".bold());
        let instructions = Title::from(Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
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
            .border_set(border::THICK);

        let counter_text = Text::from(vec![
            Line::from(vec!["Length: ".into(), self.length.to_string().yellow()]),
            Line::from(vec!["Password: ".into(), self.password.clone().yellow()]),
        ]);

        Paragraph::new(counter_text).block(block).render(area, buf);
    }
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
