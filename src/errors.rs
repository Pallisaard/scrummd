use std::error;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TeamError {
    #[error("Error reading team member on line {line_idx} - Could not read name.")]
    ReadNameError { line_idx: usize },
    #[error("Task error - Expected one of ['add', 'remove', 'create'], got {got}")]
    ReadError { got: String },
    #[error("Member {member_name} already exists in team {team_name}")]
    MemberAlreadyExists {
        member_name: String,
        team_name: String,
    },
    #[error("Member {member_name} does not exist in team {team_name}")]
    MemberDoesNotExist {
        member_name: String,
        team_name: String,
    },
    #[error("Team {name} already exists")]
    TeamAlreadyExists { name: String },
}

#[derive(Error, Debug)]
pub enum ValueError<T, U> {
    #[error("Bad value. Expected {expected}, got {got}")]
    BadValue { expected: T, got: U },
    #[error("Expected one of {expected}, got {got}")]
    BadValueList { expected: Vec<T>, got: U },
}
