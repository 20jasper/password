use rand::Rng;
use ratatui::{
    prelude::*,
    widgets::{block::Title, Paragraph},
};

use super::{styled_block, App};

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
