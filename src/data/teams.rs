use ratatui::style::Color;

macro_rules! team_art {
    ($abbrev:literal) => {
        include_str!(concat!("../../resources/art/teams/", $abbrev, ".txt"))
    };
}

/// Static team definition embedded at compile time.
pub struct TeamDef {
    pub name: &'static str,
    pub abbrev: &'static str,
    pub color: Color,
    pub art: &'static str,
}

/// All NHL teams, ordered alphabetically by full name.
pub const TEAMS: &[TeamDef] = &[
    TeamDef { name: "Anaheim Ducks",         abbrev: "ANA", color: Color::Rgb(252, 76, 2),    art: team_art!("ANA") },
    TeamDef { name: "Boston Bruins",         abbrev: "BOS", color: Color::Rgb(252, 181, 20),  art: team_art!("BOS") },
    TeamDef { name: "Buffalo Sabres",        abbrev: "BUF", color: Color::Rgb(0, 100, 210),   art: team_art!("BUF") },
    TeamDef { name: "Calgary Flames",        abbrev: "CGY", color: Color::Rgb(200, 16, 46),   art: team_art!("CGY") },
    TeamDef { name: "Carolina Hurricanes",   abbrev: "CAR", color: Color::Rgb(206, 17, 38),   art: team_art!("CAR") },
    TeamDef { name: "Chicago Blackhawks",    abbrev: "CHI", color: Color::Rgb(207, 10, 44),   art: team_art!("CHI") },
    TeamDef { name: "Colorado Avalanche",    abbrev: "COL", color: Color::Rgb(111, 56, 76),   art: team_art!("COL") },
    TeamDef { name: "Columbus Blue Jackets", abbrev: "CBJ", color: Color::Rgb(0, 80, 170),    art: team_art!("CBJ") },
    TeamDef { name: "Dallas Stars",          abbrev: "DAL", color: Color::Rgb(0, 148, 100),   art: team_art!("DAL") },
    TeamDef { name: "Detroit Red Wings",     abbrev: "DET", color: Color::Rgb(206, 17, 38),   art: team_art!("DET") },
    TeamDef { name: "Edmonton Oilers",       abbrev: "EDM", color: Color::Rgb(252, 76, 2),    art: team_art!("EDM") },
    TeamDef { name: "Florida Panthers",      abbrev: "FLA", color: Color::Rgb(200, 16, 46),   art: team_art!("FLA") },
    TeamDef { name: "Los Angeles Kings",     abbrev: "LAK", color: Color::Rgb(162, 170, 173), art: team_art!("LAK") },
    TeamDef { name: "Minnesota Wild",        abbrev: "MIN", color: Color::Rgb(30, 155, 80),   art: team_art!("MIN") },
    TeamDef { name: "Montreal Canadiens",    abbrev: "MTL", color: Color::Rgb(175, 30, 45),   art: team_art!("MTL") },
    TeamDef { name: "Nashville Predators",   abbrev: "NSH", color: Color::Rgb(255, 182, 18),  art: team_art!("NSH") },
    TeamDef { name: "New Jersey Devils",     abbrev: "NJD", color: Color::Rgb(206, 17, 38),   art: team_art!("NJD") },
    TeamDef { name: "New York Islanders",    abbrev: "NYI", color: Color::Rgb(0, 100, 180),   art: team_art!("NYI") },
    TeamDef { name: "New York Rangers",      abbrev: "NYR", color: Color::Rgb(0, 70, 200),    art: team_art!("NYR") },
    TeamDef { name: "Ottawa Senators",       abbrev: "OTT", color: Color::Rgb(200, 16, 46),   art: team_art!("OTT") },
    TeamDef { name: "Philadelphia Flyers",   abbrev: "PHI", color: Color::Rgb(247, 73, 2),    art: team_art!("PHI") },
    TeamDef { name: "Pittsburgh Penguins",   abbrev: "PIT", color: Color::Rgb(252, 181, 20),  art: team_art!("PIT") },
    TeamDef { name: "San Jose Sharks",       abbrev: "SJS", color: Color::Rgb(0, 155, 170),   art: team_art!("SJS") },
    TeamDef { name: "Seattle Kraken",        abbrev: "SEA", color: Color::Rgb(0, 155, 170),   art: team_art!("SEA") },
    TeamDef { name: "St. Louis Blues",       abbrev: "STL", color: Color::Rgb(0, 80, 190),    art: team_art!("STL") },
    TeamDef { name: "Tampa Bay Lightning",   abbrev: "TBL", color: Color::Rgb(0, 80, 200),    art: team_art!("TBL") },
    TeamDef { name: "Toronto Maple Leafs",   abbrev: "TOR", color: Color::Rgb(0, 80, 175),    art: team_art!("TOR") },
    TeamDef { name: "Utah Hockey Club",      abbrev: "UTA", color: Color::Rgb(105, 179, 231), art: team_art!("UTA") },
    TeamDef { name: "Vancouver Canucks",     abbrev: "VAN", color: Color::Rgb(0, 100, 180),   art: team_art!("VAN") },
    TeamDef { name: "Vegas Golden Knights",  abbrev: "VGK", color: Color::Rgb(185, 151, 91),  art: team_art!("VGK") },
    TeamDef { name: "Washington Capitals",   abbrev: "WSH", color: Color::Rgb(200, 16, 46),   art: team_art!("WSH") },
    TeamDef { name: "Winnipeg Jets",         abbrev: "WPG", color: Color::Rgb(85, 135, 195),  art: team_art!("WPG") },
];

