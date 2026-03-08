use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Cell, Row, Table, Widget},
};

use crate::data::NhlData;
use crate::widget::theme::{rank_color, rank_name_color};

pub struct CursorInfo {
    pub position: usize,
    pub grabbed: bool,
}

pub struct RankedTable<'a> {
    pub title: &'a str,
    pub rank_header: &'a str,
    pub teams: &'a [usize],
    pub nhl_data: &'a NhlData,
    pub offset: usize,
    pub cursor: Option<CursorInfo>,
}

impl<'a> Widget for RankedTable<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .border_style(Style::default().fg(Color::Cyan))
            .title(
                Line::from(Span::styled(
                    format!(" {} ", self.title),
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ))
                .centered(),
            );

        let inner = block.inner(area);
        block.render(area, buf);

        let visible_rows = inner.height.saturating_sub(1) as usize;

        let rows: Vec<Row> = self
            .teams
            .iter()
            .enumerate()
            .skip(self.offset)
            .take(visible_rows)
            .map(|(list_idx, &team_idx)| {
                let rank = list_idx + 1;
                let name = self.nhl_data.team_name(team_idx);

                let (rank_style, name_style, prefix) = match &self.cursor {
                    Some(c) if list_idx == c.position && c.grabbed => (
                        Style::default()
                            .fg(Color::Black)
                            .bg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                        Style::default()
                            .fg(Color::Black)
                            .bg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                        "  ↕  ",
                    ),
                    Some(c) if list_idx == c.position => (
                        Style::default()
                            .fg(Color::Black)
                            .bg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                        Style::default()
                            .fg(Color::Black)
                            .bg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                        "  ▶  ",
                    ),
                    _ => (
                        Style::default().fg(rank_color(rank)),
                        Style::default().fg(rank_name_color(rank)),
                        "     ",
                    ),
                };

                Row::new(vec![
                    Cell::from(format!("{prefix}{rank:>2}")).style(rank_style),
                    Cell::from(name.to_string()).style(name_style),
                ])
            })
            .collect();

        let widths = [Constraint::Length(10), Constraint::Min(20)];

        let table = Table::new(rows, widths).header(
            Row::new(vec![
                Cell::from(format!("  {}", self.rank_header)).style(
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
                ),
                Cell::from("Team").style(
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
                ),
            ])
            .height(1),
        );

        Widget::render(table, inner, buf);
    }
}
