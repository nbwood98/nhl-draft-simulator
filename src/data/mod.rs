pub mod logos;
pub mod nhl_api;
pub mod teams;

use nhl_api::TeamData;

/// Shared NHL data fetched at startup and held in memory.
#[derive(Debug, Clone)]
pub struct NhlData {
    pub teams: Vec<TeamData>,
    /// Pre-rendered braille art lines per team (indexed same as `teams`).
    pub logos: Vec<Vec<String>>,
}

impl NhlData {
    pub fn len(&self) -> usize {
        self.teams.len()
    }

    pub fn is_empty(&self) -> bool {
        self.teams.is_empty()
    }

    pub fn team_name(&self, idx: usize) -> &str {
        &self.teams[idx].name
    }

    pub fn team_abbrev(&self, idx: usize) -> &str {
        &self.teams[idx].abbrev
    }

    pub fn team_logo_art(&self, idx: usize) -> &[String] {
        &self.logos[idx]
    }
}

impl Default for NhlData {
    fn default() -> Self {
        // Fallback to hardcoded data if API call not made
        let teams: Vec<TeamData> = teams::NHL_TEAMS
            .iter()
            .zip(teams::NHL_ABBREVS.iter())
            .enumerate()
            .map(|(i, (name, abbrev))| TeamData {
                name: name.to_string(),
                abbrev: abbrev.to_string(),
                logo_url: String::new(),
                league_rank: (teams::NHL_TEAMS.len() - i) as u32,
            })
            .collect();
        let logos = vec![logos::placeholder_art(); teams.len()];
        Self { teams, logos }
    }
}
