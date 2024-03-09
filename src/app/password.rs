const PIN_RANGE: RangeInclusive<usize> = 3..=12;
const RANDOM_RANGE: RangeInclusive<usize> = 8..=100;

pub mod generate;
pub mod options;

use core::{fmt::Display, ops::RangeInclusive};

use ratatui::prelude::*;

use super::{list::Items, App};

#[derive(Debug, Clone, PartialEq, Eq, Copy, Default)]
pub enum PasswordType {
    #[default]
    Pin,
    Random {
        numbers: bool,
        symbols: bool,
    },
}

impl Display for PasswordType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            PasswordType::Pin => write!(f, "Pin"),
            PasswordType::Random { .. } => {
                write!(f, "Random")
            }
        }
    }
}

impl PasswordType {
    pub fn get_range(&self) -> RangeInclusive<usize> {
        match self {
            PasswordType::Pin => PIN_RANGE,
            PasswordType::Random { .. } => RANDOM_RANGE,
        }
    }
}

pub fn ui(frame: &mut Frame<'_>, app: &mut App, password_type: PasswordType) {
    app.update_password(password_type);

    match password_type {
        PasswordType::Pin => {
            generate::render(frame, frame.size(), app);
        }
        PasswordType::Random { .. } => {
            let [generator_area, options_area] =
                Layout::horizontal([Constraint::Min(80), Constraint::Length(60)])
                    .areas(frame.size());

            generate::render(frame, generator_area, app);
            options::render(frame, options_area, app);
        }
    }
}

impl Default for Items<PasswordType> {
    fn default() -> Self {
        let items = vec![
            PasswordType::Pin,
            PasswordType::Random {
                numbers: true,
                symbols: true,
            },
        ];

        Items::new(items)
    }
}
