use std::error::Error;
use std::{fs::File, io::Read, io::Write};

use crate::errors::TeamError;

#[derive(Debug)]
pub struct TeamMember {
    pub name: String,
}

impl PartialEq for TeamMember {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

// Rust alias type for a team is vec<TeamMember>
pub struct Team {
    pub name: String,
    members: Vec<TeamMember>,
}

impl Team {
    pub fn new(name: String) -> Self {
        Team {
            name,
            members: Vec::new(),
        }
    }

    pub fn contains_name(&self, name: &str) -> bool {
        self.members.contains(&TeamMember {
            name: name.to_string(),
        })
    }

    pub fn add_member(&mut self, name: String) -> Result<(), Box<dyn Error>> {
        if self.members.contains(&TeamMember { name: name.clone() }) {
            return Err(Box::new(TeamError::MemberAlreadyExists {
                member_name: name,
                team_name: "main_team".to_string(),
            }));
        }
        self.members.push(TeamMember { name });
        Ok(())
    }

    pub fn remove_member(&mut self, name: String) -> Result<(), Box<dyn Error>> {
        if !self.contains_name(&name) {
            return Err(Box::new(TeamError::MemberDoesNotExist {
                member_name: name,
                team_name: "main_team".to_string(),
            }));
        }
        self.members.retain(|x| x.name != name);
        Ok(())
    }

    pub fn save_team(&self, team_name: &str) -> Result<(), Box<dyn Error>> {
        let mut file = File::create(format!("teams/{}.txt", team_name))?;
        for member in &self.members {
            file.write_all(member.name.as_bytes())?;
            file.write_all(b"\n")?;
        }
        Ok(())
    }
}

pub fn delete_team(team_name: &str) -> Result<(), Box<dyn Error>> {
    std::fs::remove_file(team_name)?;
    Ok(())
}

// Load a team from a txt file. The file has the format of one team member per line, with the id and name separated by a comma.
pub fn load_team(team_name: &str) -> Result<Team, Box<dyn std::error::Error>> {
    let mut file = File::open(team_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut team = Team::new(team_name.to_string());
    for line in contents.lines() {
        let name = line.trim().to_string();
        team.add_member(name)?;
    }
    Ok(team)
}
