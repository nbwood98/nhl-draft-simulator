use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, List, ListItem, ListState, StatefulWidget, Widget},
};

use crate::app::MenuItem;

pub struct Menu<'a> {
    pub items: &'a [MenuItem],
    pub selected: usize,
}

impl<'a> Widget for Menu<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .border_style(Style::default().fg(Color::Cyan))
            .title(
                Line::from(Span::styled(
                    " Menu ",
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ))
                .centered(),
            );

        let items: Vec<ListItem> = self
            .items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let is_quit = *item == MenuItem::Quit;
                let label = if i == self.selected {
                    format!("  ▶  {}  ", item.label())
                } else {
                    format!("     {}  ", item.label())
                };

                let style = if i == self.selected {
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else if is_quit {
                    Style::default().fg(Color::Red)
                } else {
                    Style::default().fg(Color::White)
                };

                ListItem::new(Line::from(Span::styled(label, style)).centered())
            })
            .collect();

        let list = List::new(items).block(block);

        let mut state = ListState::default();
        state.select(Some(self.selected));

        StatefulWidget::render(list, area, buf, &mut state);
    }
}

