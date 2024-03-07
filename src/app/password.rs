const PIN_RANGE: RangeInclusive<usize> = 3..=12;
const RANDOM_RANGE: RangeInclusive<usize> = 8..=100;
const MEMORABLE_RANGE: RangeInclusive<usize> = 3..=15;

use core::{fmt::Display, ops::RangeInclusive};

use rand::Rng;
use ratatui::{
    prelude::*,
    widgets::{block::Title, Paragraph, Wrap},
};

use super::{list::Items, styled_block, App};

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
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
    pub fn get_range(&self) -> RangeInclusive<usize> {
        match self {
            PasswordType::Pin { .. } => PIN_RANGE,
            PasswordType::Random { .. } => RANDOM_RANGE,
            PasswordType::Memorable { .. } => MEMORABLE_RANGE,
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
    .wrap(Wrap { trim: true })
    .block(block);

    frame.render_widget(password_text, area);
}

impl Default for Items<PasswordType> {
    fn default() -> Self {
        let items = vec![
            PasswordType::Pin {
                length: *PIN_RANGE.start(),
            },
            PasswordType::Random {
                length: *RANDOM_RANGE.start(),
                numbers: false,
                symbols: false,
            },
            PasswordType::Memorable {
                length: *MEMORABLE_RANGE.start(),
                capitalize: false,
                full_words: false,
            },
        ];

        Items::new(items)
    }
}
