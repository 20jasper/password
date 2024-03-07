use core::{
    fmt::{write, Display},
    ops::RangeInclusive,
};

use rand::Rng;
use ratatui::{
    prelude::*,
    widgets::{block::Title, Paragraph},
};

use super::{list::Items, styled_block, App};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PasswordType {
    Pin {
        length: usize,
    },
    Random {
        length: usize,
        numbers: bool,
        symbols: bool,
    },
    Memorable {
        length: usize,
        capitalize: bool,
        full_words: bool,
    },
}

impl Display for PasswordType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            PasswordType::Pin { .. } => write!(f, "Pin"),
            PasswordType::Random { .. } => {
                write!(f, "Random")
            }
            PasswordType::Memorable { .. } => write!(f, "Memorable"),
        }
    }
}

impl Default for PasswordType {
    fn default() -> Self {
        Self::Pin { length: 8 }
    }
}

impl PasswordType {
    fn get_range(&self) -> RangeInclusive<u32> {
        match self {
            PasswordType::Pin { .. } => 3..=12,
            PasswordType::Random { .. } => 8..=100,
            PasswordType::Memorable { .. } => 3..=15,
        }
    }
}

pub fn pin(length: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| rng.gen_range(0..=9).to_string())
        .collect()
}

pub fn render(frame: &mut Frame<'_>, area: Rect, app: &App) {
    let title = Title::from(" Password Generator ".bold());
    let instructions = Title::from(Line::from(vec![
        " Decrease Length ".into(),
        "<Left>/<H>".blue().bold(),
        " Increase Length ".into(),
        "<Right>/<L>".blue().bold(),
        " Quit ".into(),
        "<Q> ".blue().bold(),
    ]));
    let block = styled_block(title, instructions);

    let password_text = Paragraph::new(vec![
        Line::from(vec!["Length: ".into(), app.length.to_string().yellow()]),
        Line::from(vec!["Password: ".into(), app.password.clone().yellow()]),
    ])
    .block(block);

    frame.render_widget(password_text, area);
}

impl Default for Items<PasswordType> {
    fn default() -> Self {
        let items = vec![
            PasswordType::Pin { length: 8 },
            PasswordType::Random {
                length: 8,
                numbers: true,
                symbols: true,
            },
            PasswordType::Memorable {
                length: 8,
                capitalize: true,
                full_words: true,
            },
        ];

        Items::new(items)
    }
}
