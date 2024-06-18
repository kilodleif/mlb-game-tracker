use std::{collections::HashMap, fs};
use std::sync::OnceLock;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CommonEntity {
    pub id: Option<u32>,
    pub name: Option<String>,
    pub full_name: Option<String>,
    pub link: Option<String>,
    pub abbreviation: Option<String>
}

#[derive(Deserialize, Debug)]
pub struct CodeDescPair {
    pub code: Option<String>,
    pub description: Option<String>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MajorLeagueTeam {
    pub id: Option<u32>,
    pub name: Option<String>,
    pub link: Option<String>,
    pub venue: Option<CommonEntity>,
    pub team_code: Option<String>,
    pub abbreviation: Option<String>,
    pub team_name: Option<String>,
    pub location_name: Option<String>,
    pub league: Option<CommonEntity>,
    pub division: Option<CommonEntity>
}

///
/// variable that stores the metadata for all major league teams.
///
static MAJOR_LEAGUE_TEAMS: OnceLock<&HashMap<u32, MajorLeagueTeam>> = OnceLock::new();

///
/// initialize all the metadata.
///
pub fn init() {
    init_major_league_teams()
    // maybe there are other metadata to init
}

///
/// get the ref for a major league team by its teamId.
///
pub fn get_major_league_team_by_id(id: &u32) -> &MajorLeagueTeam {
    MAJOR_LEAGUE_TEAMS.get()
        .expect("Team metadata not initialized!")
        .get(id)
        .expect(format!("Specific team_id: {id} not exist!").as_str())
}

///
/// initialize the metadata for major league teams.
///
fn init_major_league_teams() {
    let teams: String = fs::read_to_string("resource/major-league-teams.json").unwrap();
    let teams: Vec<MajorLeagueTeam> = serde_json::from_str(&teams).unwrap();
    let mut map = HashMap::new();
    for team in teams {
        map.insert(team.id.unwrap(), team);
    }
    let to_leak = Box::new(map);
    MAJOR_LEAGUE_TEAMS.set(Box::leak(to_leak)).unwrap()
}
