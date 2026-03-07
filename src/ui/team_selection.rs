use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Cell, Row, Table, Widget},
};

use crate::data::teams::NHL_TEAMS;
use crate::screens::team_selection::TeamSelectionState;

pub struct TeamSelectionWidget<'a> {
    pub state: &'a mut TeamSelectionState,
}

impl<'a> Widget for TeamSelectionWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Outer layout: table + footer hint bar
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)])
            .split(area);

        self.render_table(chunks[0], buf);
        render_footer(chunks[1], buf);
    }
}

impl<'a> TeamSelectionWidget<'a> {
    fn render_table(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .border_style(Style::default().fg(Color::Cyan))
            .title(
                Line::from(Span::styled(
                    " Final Standings  (1 = Best  ·  32 = Worst) ",
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ))
                .centered(),
            );

        let inner = block.inner(area);
        block.render(area, buf);
        
        let visible_rows = inner.height.saturating_sub(1) as usize;
        self.state.sync_offset(visible_rows);

        let is_grabbed = self.state.grabbed.is_some();
        let cursor = self.state.cursor;
        let offset = self.state.offset;

        let rows: Vec<Row> = self
            .state
            .teams
            .iter()
            .enumerate()
            .skip(offset)
            .take(visible_rows)
            .map(|(list_idx, &team_idx)| {
                let rank = list_idx + 1;
                let name = NHL_TEAMS[team_idx];
                let is_cursor = list_idx == cursor;

                let (rank_style, name_style, prefix) = if is_cursor && is_grabbed {
                    (
                        Style::default()
                            .fg(Color::Black)
                            .bg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                        Style::default()
                            .fg(Color::Black)
                            .bg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                        "  ↕  ",
                    )
                } else if is_cursor {
                    (
                        Style::default()
                            .fg(Color::Black)
                            .bg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                        Style::default()
                            .fg(Color::Black)
                            .bg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                        "  ▶  ",
                    )
                } else {
                    let fg = rank_color(rank);
                    (
                        Style::default().fg(fg),
                        Style::default().fg(Color::White),
                        "     ",
                    )
                };

                Row::new(vec![
                    Cell::from(format!("{prefix}{rank:>2}")).style(rank_style),
                    Cell::from(name).style(name_style),
                ])
            })
            .collect();

        let widths = [Constraint::Length(10), Constraint::Min(20)];

        let table = Table::new(rows, widths)
            .header(
                Row::new(vec![
                    Cell::from("  Finish").style(
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

fn rank_color(rank: usize) -> Color {
    match rank {
        1..=5 => Color::Yellow,
        6..=11 => Color::Green,
        12..=18 => Color::Cyan,
        19..=25 => Color::Blue,
        26..=31 => Color::Magenta,
        _ => Color::Red,
    }
}

fn render_footer(area: Rect, buf: &mut Buffer) {
    let block = Block::bordered().border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let line = Line::from(vec![
        key_span("↑/↓ j/k"),
        desc_span(" navigate   "),
        key_span("Space/Enter"),
        desc_span(" grab/drop   "),
        key_span("Esc/q"),
        desc_span(" release/back"),
    ])
    .centered();

    ratatui::widgets::Paragraph::new(line).render(inner, buf);
}

fn key_span(key: &str) -> Span<'_> {
    Span::styled(
        format!("[{key}]"),
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    )
}

fn desc_span(desc: &str) -> Span<'_> {
    Span::styled(desc, Style::default().fg(Color::DarkGray))
}




