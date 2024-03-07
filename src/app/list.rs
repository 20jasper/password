use ratatui::{
    prelude::*,
    widgets::{block::Title, List, ListState},
};

use super::styled_block;

pub fn render(frame: &mut Frame<'_>, area: Rect, state: &mut ListState) {
    let title = Title::from(" Password Types ".bold());
    let instructions = Title::from(Line::from(vec![
        " Next ".into(),
        "<Down>/<J>".blue().bold(),
        " Last ".into(),
        "<Up>/<K>".blue().bold(),
    ]));
    let block = styled_block(title, instructions);

    let items = ["Item 1", "Item 2", "Item 3"];
    let list = List::new(items)
        .block(block)
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow))
        .highlight_symbol(">> ")
        .repeat_highlight_symbol(true);

    frame.render_stateful_widget(list, area, state);
}
