pub mod art;
pub mod nhl_api;
pub mod teams;

use std::collections::HashMap;

use ratatui::style::Color;

/// Runtime-ready team data, parsed from compile-time definitions.
#[derive(Debug, Clone)]
pub struct Team {
    pub name: &'static str,
    pub abbrev: &'static str,
    pub color: Color,
    pub art_lines: Vec<&'static str>,
}

/// Central NHL data: the team registry and standings-based draft order.
#[derive(Debug, Clone)]
pub struct NhlData {
    teams: Vec<Team>,
    draft_order: Vec<usize>,
}

impl NhlData {
    /// Builds NhlData from static team definitions and optional API standings.
    ///
    /// If `standings` is provided (abbrev → league rank where 1 = best),
    /// teams are ordered worst-to-best for draft order.
    /// Otherwise, teams default to alphabetical order by name.
    pub fn new(standings: Option<HashMap<String, u32>>) -> Self {
        let teams: Vec<Team> = teams::TEAMS
            .iter()
            .map(|def| Team {
                name: def.name,
                abbrev: def.abbrev,
                color: def.color,
                art_lines: art::parse_art(def.art),
            })
            .collect();

        let mut draft_order: Vec<usize> = (0..teams.len()).collect();

        if let Some(map) = standings {
            // Sort worst-to-best (highest league rank first) for draft order
            draft_order.sort_by(|&a, &b| {
                let rank_a = map.get(teams[a].abbrev).copied().unwrap_or(0);
                let rank_b = map.get(teams[b].abbrev).copied().unwrap_or(0);
                rank_b.cmp(&rank_a)
            });
        }
        // When standings is None, teams are already alphabetical (TEAMS array order),
        // so draft_order of [0, 1, 2, ...] is alphabetical by name.

        Self { teams, draft_order }
    }

    pub fn team(&self, idx: usize) -> &Team {
        &self.teams[idx]
    }

    pub fn len(&self) -> usize {
        self.teams.len()
    }

    pub fn draft_order(&self) -> &[usize] {
        &self.draft_order
    }
}
