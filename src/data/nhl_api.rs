use std::collections::HashMap;

use serde::Deserialize;

const STANDINGS_URL: &str = "https://api-web.nhle.com/v1/standings/now";

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct StandingsResponse {
    standings: Vec<StandingEntry>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct StandingEntry {
    team_abbrev: LocalizedField,
    league_sequence: u32,
}

#[derive(Deserialize)]
struct LocalizedField {
    default: String,
}

/// Fetches current NHL standings from the API.
/// Returns a map of team abbreviation to league rank (1 = best, 32 = worst).
pub fn fetch_standings() -> Result<HashMap<String, u32>, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::limited(5))
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    let resp: StandingsResponse = client.get(STANDINGS_URL).send()?.json()?;

    let standings = resp
        .standings
        .into_iter()
        .map(|entry| (entry.team_abbrev.default, entry.league_sequence))
        .collect();

    Ok(standings)
}

