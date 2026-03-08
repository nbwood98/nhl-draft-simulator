mod app;
mod data;
mod screen;
mod widget;

use std::io;

fn main() -> io::Result<()> {
    let standings = match data::nhl_api::fetch_standings() {
        Ok(s) => {
            eprintln!("✓ Fetched standings from NHL API");
            Some(s)
        }
        Err(e) => {
            eprintln!("⚠ Could not fetch NHL standings: {e}");
            eprintln!("  Using alphabetical fallback.");
            None
        }
    };

    let nhl_data = data::NhlData::new(standings);
    let mut app = app::App::new(nhl_data);
    ratatui::run(|terminal| app.run(terminal))
}