use core::fmt;

use ratatui::{
    prelude::*,
    widgets::{block::Title, List, ListState},
};

use super::styled_block;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum Item {
    #[default]
    Pin,
    Item2,
    Item3,
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Item::Pin => write!(f, "Pin"),
            Item::Item2 => write!(f, "Item 2"),
            Item::Item3 => write!(f, "Item 3"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Items {
    items: Vec<Item>,
    state: ListState,
    selected: Option<usize>,
}

impl Default for Items {
    fn default() -> Self {
        let items = vec![Item::Pin, Item::Item2, Item::Item3];
        let mut state = ListState::default();
        state.select(Some(0));

        Self {
            items,
            state,
            selected: Some(0),
        }
    }
}

impl Items {
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

pub fn render(frame: &mut Frame<'_>, area: Rect, items: &mut Items) {
    let title = Title::from(" Password Types ".bold());
    let instructions = Title::from(Line::from(vec![
        " Next ".into(),
        "<Down>/<J>".blue().bold(),
        " Last ".into(),
        "<Up>/<K>".blue().bold(),
    ]));
    let block = styled_block(title, instructions);

    let list = List::new(
        [Item::Pin, Item::Item2, Item::Item3]
            .into_iter()
            .map(|x| x.to_string()),
    )
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
