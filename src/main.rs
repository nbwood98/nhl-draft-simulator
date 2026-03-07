mod app;
mod data;
mod screens;
mod ui;

use std::io;

fn main() -> io::Result<()> {
    // Fetch live NHL standings + logos before entering TUI.
    // Falls back to hardcoded data on failure.
    let nhl_data = match data::nhl_api::fetch_standings() {
        Ok(teams) => {
            eprintln!("✓ Fetched {} teams from NHL API", teams.len());
            let logos = data::logos::fetch_logos(&teams);
            eprintln!("✓ Fetched team logos");
            data::NhlData { teams, logos }
        }
        Err(e) => {
            eprintln!("⚠ Could not fetch NHL standings: {e}");
            eprintln!("  Using fallback data.");
            data::NhlData::default()
        }
    };

    let mut app = app::App::new(nhl_data);
    ratatui::run(|terminal| app.run(terminal))
}