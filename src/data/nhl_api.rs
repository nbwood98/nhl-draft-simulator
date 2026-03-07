use serde::Deserialize;

const STANDINGS_URL: &str = "https://api-web.nhle.com/v1/standings/now";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StandingsResponse {
    standings: Vec<StandingEntry>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StandingEntry {
    team_name: LocalizedField,
    team_abbrev: LocalizedField,
    team_logo: String,
    league_sequence: u32,
}

#[derive(Debug, Deserialize)]
struct LocalizedField {
    default: String,
}

#[derive(Debug, Clone)]
pub struct TeamData {
    pub name: String,
    pub abbrev: String,
    pub logo_url: String,
    pub league_rank: u32,
}

/// Fetches current NHL standings from the API.
/// Teams are returned sorted worst-to-best (highest league_rank first)
/// since this is a draft lottery simulator.
pub fn fetch_standings() -> Result<Vec<TeamData>, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::limited(5))
        .build()?;

    let resp: StandingsResponse = client.get(STANDINGS_URL).send()?.json()?;

    let mut teams: Vec<TeamData> = resp
        .standings
        .into_iter()
        .map(|entry| TeamData {
            name: entry.team_name.default,
            abbrev: entry.team_abbrev.default,
            logo_url: entry.team_logo,
            league_rank: entry.league_sequence,
        })
        .collect();

    // Sort worst-to-best for draft order (highest rank number = worst team = picks first)
    teams.sort_by(|a, b| b.league_rank.cmp(&a.league_rank));

    Ok(teams)
}

