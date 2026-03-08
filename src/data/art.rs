/// Raw banner ASCII art, embedded from the resources directory.
pub const BANNER_RAW: &str = include_str!("../../resources/art/banner.txt");

/// Parses raw art text into individual lines, trimming trailing empty lines.
pub fn parse_art(raw: &str) -> Vec<&str> {
    let mut lines: Vec<&str> = raw.lines().collect();
    while lines.last().is_some_and(|l| l.trim().is_empty()) {
        lines.pop();
    }
    lines
}
