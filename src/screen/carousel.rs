use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::Widget,
};

use crate::data::NhlData;

const CARD_WIDTH: usize = 11;
const CARD_HEIGHT: usize = 8;

pub struct Carousel<'a> {
    pub offset: f64,
    pub teams: &'a [usize],
    pub nhl_data: &'a NhlData,
}

impl<'a> Widget for Carousel<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.height < CARD_HEIGHT as u16 {
            return;
        }

        let n = self.nhl_data.len();
        if n == 0 {
            return;
        }

        let belt = n * CARD_WIDTH;
        let px_offset = (self.offset as usize) % belt;
        let card_top = area.y + area.height.saturating_sub(CARD_HEIGHT as u16) / 2;
        let area_w = area.width as usize;
        let first_card = px_offset / CARD_WIDTH;
        let first_card_skip = px_offset % CARD_WIDTH;
        let mut x = area.x as isize - first_card_skip as isize;
        let mut card_num = first_card;

        while x < (area.x as isize + area_w as isize) {
            let team_idx = self.teams[card_num % n];
            let team = self.nhl_data.team(team_idx);
            let color = team.color;

            let card_x_start = x.max(area.x as isize) as u16;
            let card_x_end =
                (x + CARD_WIDTH as isize).min(area.x as isize + area_w as isize) as u16;

            for (row_i, art_line) in team.art_lines.iter().enumerate() {
                let y = card_top + row_i as u16;
                if y >= area.y + area.height {
                    break;
                }
                render_card_row(buf, art_line, x, card_x_start, card_x_end, y, color);
            }

            let abbrev_y = card_top + team.art_lines.len() as u16;
            if abbrev_y < area.y + area.height {
                render_text_row(
                    buf,
                    &format!("{:^9}", team.abbrev),
                    x,
                    card_x_start,
                    card_x_end,
                    abbrev_y,
                    Style::default()
                        .fg(color)
                        .add_modifier(Modifier::BOLD),
                );
            }

            let short_name: String = team.name.chars().take(9).collect();
            let name_y = card_top + team.art_lines.len() as u16 + 1;
            if name_y < area.y + area.height {
                render_text_row(
                    buf,
                    &format!("{short_name:^9}"),
                    x,
                    card_x_start,
                    card_x_end,
                    name_y,
                    Style::default().fg(Color::DarkGray),
                );
            }

            x += CARD_WIDTH as isize;
            card_num += 1;
        }

        clear_edge_columns(buf, area);
    }
}

fn render_card_row(
    buf: &mut Buffer,
    text: &str,
    card_left: isize,
    draw_start: u16,
    draw_end: u16,
    y: u16,
    color: Color,
) {
    let chars: Vec<char> = text.chars().collect();
    for screen_x in draw_start..draw_end {
        let char_idx = (screen_x as isize - card_left) as usize;
        let ch = chars.get(char_idx).copied().unwrap_or(' ');
        let style = if ch == '*' {
            Style::default()
                .fg(color)
                .add_modifier(Modifier::BOLD)
        } else if ch == ' ' {
            Style::default()
        } else {
            Style::default().fg(color)
        };
        buf[(screen_x, y)].set_char(ch).set_style(style);
    }
}

fn render_text_row(
    buf: &mut Buffer,
    text: &str,
    card_left: isize,
    draw_start: u16,
    draw_end: u16,
    y: u16,
    style: Style,
) {
    let chars: Vec<char> = text.chars().collect();
    for screen_x in draw_start..draw_end {
        let char_idx = (screen_x as isize - card_left) as usize;
        let ch = chars.get(char_idx).copied().unwrap_or(' ');
        buf[(screen_x, y)].set_char(ch).set_style(style);
    }
}

/// Clears the leftmost and rightmost columns to clip partially-visible cards.
fn clear_edge_columns(buf: &mut Buffer, area: Rect) {
    if area.width < 2 {
        return;
    }
    for row in area.y..area.y + area.height {
        buf[(area.x, row)].set_char(' ');
        buf[(area.x + area.width - 1, row)].set_char(' ');
    }
}
