use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph, Widget},
};

use crate::ui::carousel::Carousel;

/// Renders the NHL Draft header panel.
pub struct Header<'a> {
    pub carousel_offset: f64,
    pub team_order: &'a [usize],
}

/// Single unified block-character art for "NHL DRAFT".
const ART: &[&str] = &[
    "███╗   ██╗██╗  ██╗██╗      ██████╗ ██████╗  █████╗ ███████╗████████╗",
    "████╗  ██║██║  ██║██║      ██╔══██╗██╔══██╗██╔══██╗██╔════╝╚══██╔══╝",
    "██╔██╗ ██║███████║██║      ██║  ██║██████╔╝███████║█████╗     ██║   ",
    "██║╚██╗██║██╔══██║██║      ██║  ██║██╔══██╗██╔══██║██╔══╝     ██║   ",
    "██║ ╚████║██║  ██║███████╗ ██████╔╝██║  ██║██║  ██║██║        ██║   ",
    "╚═╝  ╚═══╝╚═╝  ╚═╝╚══════╝ ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝        ╚═╝   ",
];

/// Gradient colour stops (R, G, B): cyan → royal blue → magenta → gold.
const GRADIENT: &[(u8, u8, u8)] = &[
    (0, 230, 255),
    (0, 120, 255),
    (180, 0, 255),
    (255, 200, 0),
];

fn gradient_color(t: f32) -> Color {
    let stops = GRADIENT.len() - 1;
    let scaled = t * stops as f32;
    let lo = (scaled as usize).min(stops - 1);
    let hi = lo + 1;
    let frac = scaled - lo as f32;
    let (r0, g0, b0) = GRADIENT[lo];
    let (r1, g1, b1) = GRADIENT[hi];
    let lerp = |a: u8, b: u8| (a as f32 + (b as f32 - a as f32) * frac) as u8;
    Color::Rgb(lerp(r0, r1), lerp(g0, g1), lerp(b0, b1))
}

impl<'a> Widget for Header<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let outer_block = Block::bordered()
            .border_style(Style::default().fg(Color::Cyan))
            .title(
                Line::from(vec![
                    Span::styled("🏒 ", Style::default()),
                    Span::styled(
                        " NHL DRAFT LOTTERY ",
                        Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(" 🏒", Style::default()),
                ])
                .centered(),
            );

        let inner = outer_block.inner(area);
        outer_block.render(area, buf);

        // Split inner into: banner (art rows + padding) and carousel.
        let banner_height = ART.len() as u16 + 2; // art + 1 blank above + 1 below
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(banner_height),
                Constraint::Min(0),
            ])
            .split(inner);

        // --- Banner ---
        let max_chars = ART.iter().map(|r| r.chars().count()).max().unwrap_or(1);
        let mut lines: Vec<Line> = vec![Line::raw("")];
        for row in ART {
            let spans: Vec<Span> = row
                .chars()
                .enumerate()
                .map(|(i, ch)| {
                    let t = i as f32 / (max_chars - 1).max(1) as f32;
                    Span::styled(
                        ch.to_string(),
                        Style::default().fg(gradient_color(t)).add_modifier(Modifier::BOLD),
                    )
                })
                .collect();
            lines.push(Line::from(spans).centered());
        }
        Paragraph::new(lines).render(chunks[0], buf);

        // --- Carousel ---
        Carousel {
            offset: self.carousel_offset,
            teams: self.team_order,
        }
        .render(chunks[1], buf);
    }
}
