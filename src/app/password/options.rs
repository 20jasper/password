use core::fmt::{self, Display};

use super::PasswordType;
use crate::app::list::{self, Items};
use crate::app::{App, Screens};
use ratatui::{
    prelude::{Frame, *},
    widgets::block::Title,
};

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Options {
    Numbers(bool),
    Symbols(bool),
}

impl Display for Options {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Options::Numbers(b) => write!(f, "Numbers: {b}"),
            Options::Symbols(b) => write!(f, "Symbols: {b}"),
        }
    }
}

pub fn render(frame: &mut Frame<'_>, area: Rect, app: &mut App) {
    let instructions = Title::from(Line::from(vec![
        " Next ".into(),
        "<Down>/<J>".blue().bold(),
        " Last ".into(),
        "<Up>/<K>".blue().bold(),
        " Select ".into(),
        "<Enter>/<Space>".blue().bold(),
    ]));

    if let Screens::Password(PasswordType::Random { state, .. }) = &mut app.screen {
        let list = list::styled(" Options ", instructions, state);
        frame.render_stateful_widget(list, area, &mut state.state);
    }
}

impl Default for Items<Options> {
    fn default() -> Self {
        let items = vec![Options::Numbers(true), Options::Symbols(true)];

        Items::new(items)
    }
}
