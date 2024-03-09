const PIN_RANGE: RangeInclusive<usize> = 3..=12;
const RANDOM_RANGE: RangeInclusive<usize> = 8..=100;
const MEMORABLE_RANGE: RangeInclusive<usize> = 3..=15;

const NUMBERS_RANGE: RangeInclusive<char> = '1'..='9';
const ALPHABET_LOWER_RANGE: RangeInclusive<char> = 'a'..='z';
const ALPHABET_UPPER_RANGE: RangeInclusive<char> = 'A'..='Z';
const SYMBOLS_RANGE: RangeInclusive<char> = '!'..='/';

use core::{fmt::Display, iter, ops::RangeInclusive};

use rand::Rng;
use ratatui::{
    prelude::*,
    widgets::{block::Title, Paragraph, Wrap},
};

use super::{list::Items, styled_block, App};

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum PasswordType {
    Pin,
    Random { numbers: bool, symbols: bool },
    // Memorable {
    //     capitalize: bool,
    //     full_words: bool,
    // },
}

impl Display for PasswordType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            PasswordType::Pin => write!(f, "Pin"),
            PasswordType::Random { .. } => {
                write!(f, "Random")
            } // PasswordType::Memorable { .. } => write!(f, "Memorable"),
        }
    }
}

impl Default for PasswordType {
    fn default() -> Self {
        Self::Pin
    }
}

impl PasswordType {
    pub fn get_range(&self) -> RangeInclusive<usize> {
        match self {
            PasswordType::Pin { .. } => PIN_RANGE,
            PasswordType::Random { .. } => RANDOM_RANGE,
            // PasswordType::Memorable { .. } => MEMORABLE_RANGE,
        }
    }
    pub fn generate(&self, length: usize) -> String {
        match self {
            PasswordType::Pin => get_random_string(length, true, false, false),
            PasswordType::Random { numbers, symbols } => {
                get_random_string(length, *numbers, *symbols, true)
            } // PasswordType::Memorable {
              //     length,
              //     capitalize,
              //     full_words,
              // } => todo!(),
        }
    }
}

pub fn generate_random(length: usize, numbers: bool, symbols: bool, letters: bool) -> String {
    get_random_string(length, numbers, symbols, letters)
}

/// generates a random string of length `length` with at least one character
/// from each of the options marked true
fn get_random_string(length: usize, numbers: bool, symbols: bool, letters: bool) -> String {
    let mut ranges = vec![];
    if numbers {
        ranges.push(NUMBERS_RANGE);
    }
    if letters {
        ranges.push(ALPHABET_LOWER_RANGE);
        ranges.push(ALPHABET_UPPER_RANGE);
    }

    (0..ranges.len())
        .chain(iter::from_fn(|| {
            Some(rand::thread_rng().gen_range(0..ranges.len()))
        }))
        .map(|i| rand::thread_rng().gen_range(ranges[i].clone()))
        .take(length)
        .collect::<String>()
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
            PasswordType::Pin,
            PasswordType::Random {
                numbers: false,
                symbols: false,
            },
            // PasswordType::Memorable {
            //     length: *MEMORABLE_RANGE.start(),
            //     capitalize: false,
            //     full_words: false,
            // },
        ];

        Items::new(items)
    }
}
