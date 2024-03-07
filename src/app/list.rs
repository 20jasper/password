use core::fmt::Display;

use ratatui::{
    prelude::*,
    widgets::{block::Title, List, ListState},
};

use super::styled_block;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Items<T> {
    pub items: Vec<T>,
    state: ListState,
    pub selected: Option<usize>,
}

impl<T> Items<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self {
            items,
            state: ListState::default().with_selected(Some(0)),
            selected: Some(0),
        }
    }

    pub fn select(&mut self, index: usize) {
        self.selected = Some(index);
        self.state.select(Some(index));
    }

    pub fn next(&mut self) {
        self.selected = self.selected.map(|x| {
            if x == self.items.len().saturating_sub(1) {
                0
            } else {
                x.saturating_add(1)
            }
        });

        self.state.select(self.selected);
    }

    pub fn previous(&mut self) {
        self.selected = self
            .selected
            .map(|x| x.wrapping_sub(1).min(self.items.len().saturating_sub(1)));
        self.state.select(self.selected);
    }
}

pub fn render(frame: &mut Frame<'_>, area: Rect, items: &mut Items<impl Display>) {
    let title = Title::from(" Password Types ".bold());
    let instructions = Title::from(Line::from(vec![
        " Next ".into(),
        "<Down>/<J>".blue().bold(),
        " Last ".into(),
        "<Up>/<K>".blue().bold(),
    ]));
    let block = styled_block(title, instructions);

    let list = List::new(items.items.iter().map(std::string::ToString::to_string))
        .block(block)
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow))
        .highlight_symbol(">> ")
        .repeat_highlight_symbol(true);

    frame.render_stateful_widget(list, area, &mut items.state);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_wrap_sub() {
        let mut items = Items::default();

        items.select(0);
        items.previous();

        assert_eq!(items.selected, Some(2));
    }

    #[test]
    fn should_wrap_add() {
        let mut items = Items::default();

        items.select(2);
        items.next();

        assert_eq!(items.selected, Some(0));
    }
}
