use ratatui::{
    prelude::{Frame, *},
    widgets::{block::Title, Paragraph, Wrap},
};

use crate::app::{styled_block, App};

pub fn render(frame: &mut Frame<'_>, area: Rect, app: &mut App) {
    let title = Title::from(" Options ".bold());
    let instructions = Title::from(Line::from(vec![
        " Next ".into(),
        "<Down>/<J>".blue().bold(),
        " Last ".into(),
        "<Up>/<K>".blue().bold(),
        " Select ".into(),
        "<Enter>/<Space>".blue().bold(),
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
