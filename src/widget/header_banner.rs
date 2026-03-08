use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use crate::data::art;

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

/// Height of the banner in terminal rows (art lines + 1 blank above + 1 below).
pub const BANNER_HEIGHT: u16 = 8;

pub struct HeaderBanner;

impl Widget for HeaderBanner {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let art_lines = art::parse_art(art::BANNER_RAW);
        let max_chars = art_lines.iter().map(|r| r.chars().count()).max().unwrap_or(1);
        let mut lines: Vec<Line> = vec![Line::raw("")];
        for row in &art_lines {
            let row_len = row.chars().count();
            let mut spans: Vec<Span> = row
                .chars()
                .enumerate()
                .map(|(i, ch)| {
                    let t = i as f32 / (max_chars - 1).max(1) as f32;
                    Span::styled(
                        ch.to_string(),
                        Style::default()
                            .fg(gradient_color(t))
                            .add_modifier(Modifier::BOLD),
                    )
                })
                .collect();
            // Pad shorter lines to max_chars so all lines center at the same x position.
            if row_len < max_chars {
                spans.push(Span::raw(" ".repeat(max_chars - row_len)));
            }
            lines.push(Line::from(spans).centered());
        }
        Paragraph::new(lines).render(area, buf);
    }
}
