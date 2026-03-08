use ratatui::style::Color;

pub fn rank_color(rank: usize) -> Color {
    match rank {
        1..=3 => Color::Magenta,
        4..=10 => Color::Blue,
        11..=16 => Color::Cyan,
        _ => Color::Red,
    }
}

pub fn rank_name_color(rank: usize) -> Color {
    match rank {
        1..=16 => Color::White,
        _ => Color::Gray,
    }
}
