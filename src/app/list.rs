use core::fmt::Display;
use std::string::ToString;

use ratatui::{
    prelude::*,
    widgets::{block::Title, List, ListState},
};

use super::styled_block;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Items<T> {
    pub items: Vec<T>,
    pub state: ListState,
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

    pub fn get_selected(&self) -> Option<&T> {
        Some(&self.items[self.selected?])
    }

    pub fn next(&mut self) {
        self.selected = self
            .selected
            .map(|x| x.saturating_add(1) % self.items.len());

        self.state.select(self.selected);
    }

    pub fn previous(&mut self) {
        self.selected = self.selected.map(|x| {
            x.wrapping_sub(1)
                .min(self.items.len() - 1)
        });
        self.state.select(self.selected);
    }
}

pub fn ui(frame: &mut Frame<'_>, items: &mut Items<impl Display>) {
    let area = frame.size();
    render(frame, area, items);
}

pub fn styled<'a>(
    title: &'a str,
    instructions: Title<'a>,
    items: &Items<impl Display>,
) -> List<'a> {
    let title = Title::from(title.bold());
    let block = styled_block(title, instructions);

    List::new(
        items
            .items
            .iter()
            .map(ToString::to_string),
    )
    .block(block)
    .style(Style::default().fg(Color::White))
    .highlight_style(Style::default().fg(Color::Yellow))
    .highlight_symbol(">> ")
    .repeat_highlight_symbol(true)
}

pub fn render(frame: &mut Frame<'_>, area: Rect, items: &mut Items<impl Display>) {
    let instructions = Title::from(Line::from(vec![
        " Next ".into(),
        "<Down>/<J>".blue().bold(),
        " Last ".into(),
        "<Up>/<K>".blue().bold(),
        " Select ".into(),
        "<Enter>/<Space>".blue().bold(),
        " Quit ".into(),
        "<Q> ".blue().bold(),
    ]));
    let list = styled(" Password Types ", instructions, items);

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
