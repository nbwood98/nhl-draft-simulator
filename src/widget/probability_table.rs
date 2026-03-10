use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Cell, Row, Table, Widget},
};

use crate::data::NhlData;

pub struct ProbabilityTable<'a> {
    pub pick_counts: &'a [[u32; 16]; 16],
    pub completed: u32,
    pub team_order: &'a [usize],
    pub nhl_data: &'a NhlData,
}

impl<'a> Widget for ProbabilityTable<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .border_style(Style::default().fg(Color::Cyan))
            .title(
                Line::from(Span::styled(
                    " Pick Probability (%) by Seed ",
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ))
                .centered(),
            );

        let inner = block.inner(area);
        block.render(area, buf);

        let header_style = Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD | Modifier::UNDERLINED);

        let mut header_cells = vec![
            Cell::from("  # ").style(header_style),
            Cell::from("Team").style(header_style),
        ];
        for pick in 1..=16u8 {
            header_cells.push(Cell::from(format!("{pick:>4}")).style(header_style));
        }

        let rows: Vec<Row> = (0..16)
            .map(|seed| {
                let team_idx = self.team_order[seed];
                let abbrev = self.nhl_data.team(team_idx).abbrev;

                let mut cells = vec![
                    Cell::from(format!(" {:>2} ", seed + 1))
                        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                    Cell::from(abbrev.to_string())
                        .style(Style::default().fg(Color::White)),
                ];

                for pick in 0..16 {
                    let count = self.pick_counts[seed][pick];
                    if self.completed == 0 || count == 0 {
                        cells.push(Cell::from(""));
                    } else {
                        let pct = count as f64 / self.completed as f64 * 100.0;
                        let (text, color) = format_probability(pct);
                        cells.push(Cell::from(text).style(Style::default().fg(color)));
                    }
                }

                Row::new(cells)
            })
            .collect();

        let mut widths = vec![Constraint::Length(4), Constraint::Length(5)];
        for _ in 0..16 {
            widths.push(Constraint::Length(5));
        }

        let table = Table::new(rows, widths).header(Row::new(header_cells).height(1));
        Widget::render(table, inner, buf);
    }
}

fn format_probability(pct: f64) -> (String, Color) {
    let color = if pct >= 20.0 {
        Color::Green
    } else if pct >= 5.0 {
        Color::Yellow
    } else if pct >= 1.0 {
        Color::White
    } else {
        Color::DarkGray
    };

    let text = if pct >= 99.95 {
        " 100".to_string()
    } else {
        format!("{pct:>4.1}")
    };

    (text, color)
}
