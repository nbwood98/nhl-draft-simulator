use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::Widget,
};

use crate::data::teams::{NHL_ABBREVS, NHL_TEAMS};

// ---------------------------------------------------------------------------
// Placeholder "logo" cards — 5 rows × 9 cols of ASCII art per team.
// Each team gets a unique geometric shape built from its abbreviation.
// ---------------------------------------------------------------------------

/// Returns a 5-line ASCII art card for a team by its index.
fn placeholder_card(team_idx: usize) -> [&'static str; 5] {
    // We cycle through a set of frame shapes and embed the abbreviation.
    // The shapes rotate so adjacent teams look distinct.
    match team_idx % 8 {
        0 => [" ╔═══╗ ", " ║   ║ ", " ║ * ║ ", " ║   ║ ", " ╚═══╝ "],
        1 => [" ╭───╮ ", " │   │ ", " │ * │ ", " │   │ ", " ╰───╯ "],
        2 => ["  ▄▄▄  ", " █   █ ", " █ * █ ", " █   █ ", "  ▀▀▀  "],
        3 => [" ┌───┐ ", " │   │ ", " │ * │ ", " │   │ ", " └───┘ "],
        4 => ["  /‾‾\\ ", " /    \\", "| *   |", " \\    /", "  \\__/ "],
        5 => [" ╔═══╗ ", " ╠   ╣ ", " ╠ * ╣ ", " ╠   ╣ ", " ╚═══╝ "],
        6 => [" ┏━━━┓ ", " ┃   ┃ ", " ┃ * ┃ ", " ┃   ┃ ", " ┗━━━┛ "],
        _ => [" +---+ ", " |   | ", " | * | ", " |   | ", " +---+ "],
    }
}

/// Accent colour cycling through NHL-ish palette per team.
fn team_color(team_idx: usize) -> Color {
    const PALETTE: &[Color] = &[
        Color::Cyan,
        Color::LightBlue,
        Color::Magenta,
        Color::LightYellow,
        Color::Green,
        Color::LightRed,
        Color::LightMagenta,
        Color::LightGreen,
    ];
    PALETTE[team_idx % PALETTE.len()]
}

// ---------------------------------------------------------------------------
// Widget
// ---------------------------------------------------------------------------

/// Card width in terminal columns (logo 7 + 2 padding).
pub const CARD_WIDTH: usize = 11;
/// Card height: 5 art lines + 2 text lines (abbrev + name) + 1 gap = 8.
pub const CARD_HEIGHT: usize = 8;

pub struct Carousel<'a> {
    /// Continuous horizontal scroll offset (in columns, sub-integer for smooth motion).
    pub offset: f64,
    /// Abbreviations to display (same order as NHL_TEAMS).
    pub teams: &'a [usize], // indices into NHL_TEAMS; pass app.team_selection.teams
}

impl<'a> Widget for Carousel<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.height < CARD_HEIGHT as u16 {
            return;
        }

        let n = NHL_TEAMS.len();
        // Total belt width — cards tile seamlessly.
        let belt = n * CARD_WIDTH;
        // Integer pixel offset, wrapped so the belt loops.
        let px_offset = (self.offset as usize) % belt;

        // Vertical centre of the card block within `area`.
        let card_top = area.y + area.height.saturating_sub(CARD_HEIGHT as u16) / 2;

        // We'll render enough cards to fill the area width, starting just before
        // the left edge so the scroll looks seamless.
        let area_w = area.width as usize;

        // Work out which card is first (possibly partially off-screen to the left).
        let first_card = px_offset / CARD_WIDTH;
        let first_card_skip = px_offset % CARD_WIDTH; // cols already scrolled past

        let mut x = area.x as isize - first_card_skip as isize;
        let mut card_num = first_card;

        while x < (area.x as isize + area_w as isize) {
            let team_idx = self.teams[card_num % n];
            let abbrev = NHL_ABBREVS[team_idx];
            let name = NHL_TEAMS[team_idx];
            let art = placeholder_card(team_idx);
            let color = team_color(team_idx);

            // How many columns of this card are visible?
            let card_x_start = x.max(area.x as isize) as u16;
            let card_x_end = (x + CARD_WIDTH as isize).min(area.x as isize + area_w as isize) as u16;

            // Draw each art row.
            for (row_i, art_line) in art.iter().enumerate() {
                let y = card_top + row_i as u16;
                if y >= area.y + area.height {
                    break;
                }
                render_card_row(buf, art_line, x, card_x_start, card_x_end, y, color);
            }

            // Abbrev line (centred in card).
            let abbrev_y = card_top + 5;
            if abbrev_y < area.y + area.height {
                render_text_row(
                    buf,
                    &format!("{abbrev:^9}"),
                    x,
                    card_x_start,
                    card_x_end,
                    abbrev_y,
                    Style::default().fg(color).add_modifier(Modifier::BOLD),
                );
            }

            // Short team name (truncated to 9 chars, centred).
            let short_name: String = name.chars().take(9).collect();
            let name_y = card_top + 6;
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

        // Fading edge vignettes — overwrite leftmost/rightmost columns with darkness.
        vignette(buf, area);
    }
}

/// Render one row of card art, clipping to [card_x_start, card_x_end).
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

/// Render a text row the same way (styled uniformly).
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

/// Darken the left and right edges for a "fade out" vignette effect.
fn vignette(buf: &mut Buffer, area: Rect) {
    let fade_cols = (area.width / 6).max(2);
    for col in 0..fade_cols {
        let alpha = col as f32 / fade_cols as f32;
        let dim = if alpha < 0.4 {
            Color::Black
        } else {
            Color::DarkGray
        };
        _ = dim; // we just blank the cell char to create darkness
        for row in area.y..area.y + area.height {
            // Left fade
            let lx = area.x + col;
            if lx < area.x + area.width {
                if col == 0 {
                    buf[(lx, row)].set_char(' ');
                }
            }
            // Right fade
            let rx = area.x + area.width - 1 - col;
            if rx >= area.x {
                if col == 0 {
                    buf[(rx, row)].set_char(' ');
                }
            }
        }
    }
}


