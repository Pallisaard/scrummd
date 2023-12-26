use std::path::Path;

pub fn get_teams_dir(team: &str) -> String {
    format!("teams/{}.txt", team)
}

pub fn team_exist(team: &str) -> bool {
    let team_path = get_teams_dir(team);
    Path::new(&team_path).exists()
}
