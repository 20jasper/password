const PIN_RANGE: RangeInclusive<usize> = 3..=12;
const RANDOM_RANGE: RangeInclusive<usize> = 8..=100;

mod generate;

use core::{fmt::Display, ops::RangeInclusive};

use ratatui::{
    prelude::*,
    widgets::{block::Title, Paragraph, Wrap},
};

use super::{list::Items, styled_block, App};

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

    let area = frame.size();
    render_generator(frame, area, app);
}

fn render_generator(frame: &mut Frame<'_>, area: Rect, app: &App) {
    let title = Title::from(" Password Generator ".bold());
    let instructions = Title::from(Line::from(vec![
        " Decrease Length ".into(),
        "<Left>/<H>".blue().bold(),
        " Increase Length ".into(),
        "<Right>/<L>".blue().bold(),
        " Back to List ".into(),
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
                numbers: true,
                symbols: true,
            },
        ];

        Items::new(items)
    }
}
