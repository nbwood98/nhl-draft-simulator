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
    pub initial_order: Option<&'a [usize]>,
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

        let show_diff = self.initial_order.is_some();

        let rows: Vec<Row> = self
            .teams
            .iter()
            .enumerate()
            .skip(self.offset)
            .take(visible_rows)
            .map(|(list_idx, &team_idx)| {
                let rank = list_idx + 1;
                let name = self.nhl_data.team(team_idx).name;

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

                let mut cells = vec![
                    Cell::from(format!("{prefix}{rank:>2}")).style(rank_style),
                ];

                if let Some(initial) = self.initial_order {
                    let original_pos = initial.iter().position(|&t| t == team_idx).unwrap_or(list_idx);
                    let diff = original_pos as isize - list_idx as isize;

                    let diff_cell = if diff > 0 {
                        Cell::from(format!(" ▲ {}", diff))
                            .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
                    } else if diff < 0 {
                        Cell::from(format!(" ▼ {}", diff.abs()))
                            .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
                    } else {
                        Cell::from("   -")
                            .style(Style::default().fg(Color::DarkGray))
                    };
                    cells.push(diff_cell);
                }

                cells.push(Cell::from(name.to_string()).style(name_style));

                Row::new(cells)
            })
            .collect();

        let (widths, header): (Vec<Constraint>, Vec<Cell>) = if show_diff {
            (
                vec![Constraint::Length(10), Constraint::Length(6), Constraint::Min(20)],
                vec![
                    Cell::from(format!("  {}", self.rank_header)).style(
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
                    ),
                    Cell::from("+/-").style(
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
                    ),
                    Cell::from("Team").style(
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
                    ),
                ],
            )
        } else {
            (
                vec![Constraint::Length(10), Constraint::Min(20)],
                vec![
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
                ],
            )
        };

        let table = Table::new(rows, widths).header(
            Row::new(header).height(1),
        );

        Widget::render(table, inner, buf);
    }
}
