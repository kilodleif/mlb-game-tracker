use serde::Deserialize;
use crate::metadata::{self, CommonEntity};

const CODED_GAME_STATE_IN_PROGRESS: &str = "I";
const CODED_GAME_STATE_FINAL: &str = "F";
const CODED_GAME_STATE_POSTPONED: &str = "D";
const CODED_GAME_STATE_SCHEDULED: &str = "S";

#[derive(Deserialize, Debug)]
pub struct Schedule {
    pub dates: Option<Vec<DateSchedule>>
}

impl Schedule {

    pub fn print_current_date_schedule(&self) {
        let current_date_schedule = self.get_current_date_schedule();

        current_date_schedule.print_game_date();

        current_date_schedule.print_game_finished();

        current_date_schedule.print_game_in_progress();

        current_date_schedule.print_game_postponed();

        current_date_schedule.print_game_scheduled()
    }

    fn get_current_date_schedule(&self) -> &DateSchedule {
        self.dates.as_ref().unwrap().get(0).unwrap()
    }

}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DateSchedule {
    pub date: Option<String>,
    pub total_items: Option<u8>,
    pub total_events: Option<u8>,
    pub total_games: Option<u8>,
    pub total_games_in_progress: Option<u8>,
    pub games: Option<Vec<Game>>
}

impl DateSchedule {

    fn print_game_date(&self) {
        println!("[Game Date: {}]", self.date.as_ref().unwrap())
    }

    fn print_game_finished(&self) {
        println!("<Games Finished>");
        self.games.as_ref().unwrap().iter()
            .filter(|game| game.game_status_eq(CODED_GAME_STATE_FINAL))
            .for_each(|game|game.print_game_scores());
        println!()
    }

    fn print_game_in_progress(&self) {
        println!("<Games In Progress>");
        self.games.as_ref().unwrap().iter()
            .filter(|game| game.game_status_eq(CODED_GAME_STATE_IN_PROGRESS))
            .enumerate()
            .for_each(|(idx, game)|game.print_game_scores_with_index(idx));
        println!()
    }

    fn print_game_postponed(&self) {
        println!("<Games Postponed>");
        self.games.as_ref().unwrap().iter()
            .filter(|game| game.game_status_eq(CODED_GAME_STATE_POSTPONED))
            .for_each(|game|game.print_game_scores());
        println!()
    }

    fn print_game_scheduled(&self) {
        println!("<Games Scheduled>");
        self.games.as_ref().unwrap().iter()
            .filter(|game| game.game_status_eq(CODED_GAME_STATE_SCHEDULED))
            .for_each(|game|game.print_game_versus());
        println!()
    }

}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub game_pk: Option<u32>,
    pub link: Option<String>,
    pub game_type: Option<String>,
    pub season: Option<String>,
    pub game_date: Option<String>,
    pub status: Option<GameStatus>,
    pub teams: Option<GamePlayTeams>
}

impl Game {

    fn game_status_eq(&self, coded_game_state: &str) -> bool {
        self.status.as_ref().unwrap().coded_game_state.as_ref().unwrap().eq(coded_game_state)
    }

    fn print_game_versus(&self) {
        let game_play_teams = self.teams.as_ref().unwrap();
        let away_team = game_play_teams.away.as_ref().unwrap();
        let home_team = game_play_teams.home.as_ref().unwrap();
        let away_team_abbr = away_team.get_team_abbr();
        let home_team_abbr = home_team.get_team_abbr();
        println!(" {away_team_abbr:3}  vs  {home_team_abbr:3}")
    }
    
    fn print_game_scores(&self) {
        let game_play_teams = self.teams.as_ref().unwrap();
        let (away_team_abbr, home_team_abbr,
            away_team_score, home_team_score) = game_play_teams.get_away_home_abbrs_and_scores();
        println!(" {away_team_abbr:3} {away_team_score:2} : {home_team_score:2} {home_team_abbr:3}")
    }

    fn print_game_scores_with_index(&self, idx: usize) {
        let game_play_teams = self.teams.as_ref().unwrap();
        let (away_team_abbr, home_team_abbr,
            away_team_score, home_team_score) = game_play_teams.get_away_home_abbrs_and_scores();
        println!(" {idx}. {away_team_abbr:3} {away_team_score:2} : {home_team_score:2} {home_team_abbr:3}")
    }
    


}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GameStatus {
    abstract_game_state: Option<String>,
    coded_game_state: Option<String>,
    detailed_state: Option<String>,
    status_code: Option<String>,
    start_time_TBD: Option<bool>,
    abstract_game_code: Option<String>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GamePlayTeams {
    pub away: Option<GamePlayTeam>,
    pub home: Option<GamePlayTeam>
}

impl GamePlayTeams {
    fn get_away_home_abbrs_and_scores(&self) -> (&String, &String, u8, u8) {
        let away_team = self.away.as_ref().unwrap();
        let home_team = self.home.as_ref().unwrap();
        let away_team_abbr = away_team.get_team_abbr();
        let home_team_abbr = home_team.get_team_abbr();
        let away_team_score = away_team.score.unwrap();
        let home_team_score = home_team.score.unwrap();
        (away_team_abbr, home_team_abbr, away_team_score, home_team_score)
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GamePlayTeam {
    pub league_record: Option<TeamRecord>,
    pub score: Option<u8>,
    pub team: Option<CommonEntity>,
    pub is_winner: Option<bool>,
    pub series_number: Option<u32>
}

impl GamePlayTeam {
    fn get_team_abbr(&self) -> &String {
        let team_abbr = self.team.as_ref().unwrap().id.as_ref().unwrap();
        let team_abbr = metadata::get_major_league_team_by_id(team_abbr);
        team_abbr.abbreviation.as_ref().unwrap()
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TeamRecord {
    pub wins: Option<u8>,
    pub losses: Option<u8>,
    pub pct: Option<String>
}