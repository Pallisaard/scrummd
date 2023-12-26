use argparse::{ArgumentParser, Store};
use errors::TeamError;
use std::error::Error;

mod errors;
mod team;

#[derive(Debug)]
enum BaseCmd {
    Team(String, TeamCmd),
}

#[derive(Debug)]
enum TeamCmd {
    AddMember { name: String },
    RemoveMember { name: String },
    CreateTeam { name: String },
    DeleteTeam { name: String },
}

fn main() -> Result<(), Box<dyn Error>> {
    // let mut verbose = false;
    let mut program_cmd_str = String::new();
    let mut task_cmd_str = String::new();
    let mut team_name = "main_team".to_string();
    let mut member_name = String::new();

    // let mut name = "World".to_string();
    {
        // this block limits scope of borrows by parser.refer() method
        let mut parser = ArgumentParser::new();
        parser.set_description("strummd.");
        //     parser.refer(&mut verbose)
        //         .add_option(&["-v", "--verbose"], StoreTrue,
        //         "Be verbose");
        //     parser.parse_args_or_exit();
        parser
            .refer(&mut program_cmd_str)
            .add_argument("primary command", Store, "Command to run");
        parser
            .refer(&mut task_cmd_str)
            .add_argument("task command", Store, "Command to run");
        // parser
        //     .refer(&mut team_name)
        //     .add_argument("team name", Store, "The name of the team");
        parser.refer(&mut member_name).add_argument(
            "member name",
            Store,
            "The name of the team member to be actioned on",
        );
        parser
            .refer(&mut team_name)
            .add_option(&["--team"], Store, "The team name.");
        parser.parse_args_or_exit();
    }

    let program_cmd = match program_cmd_str.as_str() {
        "team" => {
            let task_cmd = parse_team_cmd(member_name, task_cmd_str)?;
            BaseCmd::Team(team_name, task_cmd)
        }
        v => return Err(Box::new(TeamError::ReadError { got: v.to_string() })),
    };

    println!("Running command: {:?}", program_cmd);
    run_cmd(program_cmd)?;
    Ok(())
}

fn run_cmd(cmd: BaseCmd) -> Result<(), Box<dyn Error>> {
    match cmd {
        BaseCmd::Team(team_name, task_cmd) => {
            handle_team_cmd(task_cmd, team_name)?;
        }
    }
    Ok(())
}

fn handle_team_cmd(cmd: TeamCmd, team_name: String) -> Result<(), Box<dyn Error>> {
    match cmd {
        TeamCmd::AddMember { name } => {
            let team_filename = format!("teams/{}.txt", team_name);
            let mut team = team::load_team(&team_filename)?;
            team.add_member(name.clone())?;
            team.save_team(&team_name)?;
            println!("Added member {:?} to team {:?}", name, team.name);
        }
        TeamCmd::RemoveMember { name } => {
            let team_filename = format!("{}.txt", team_name);
            let mut team = team::load_team(&team_filename)?;
            team.remove_member(name.clone())?;
            team.save_team(&team_name)?;
            println!("Removed member {:?} from team {:?}", name, team.name);
        }
        TeamCmd::CreateTeam { name } => {
            create_team(name.clone())?;
            println!("Created team {:?}", name);
        }
        TeamCmd::DeleteTeam { name } => {
            delete_team(name.clone())?;
            println!("Deleted team {:?}", name);
        }
    }
    Ok(())
}

fn parse_team_cmd(member_name: String, task_cmd_str: String) -> Result<TeamCmd, Box<dyn Error>> {
    match task_cmd_str.as_str() {
        "add" => Ok(TeamCmd::AddMember { name: member_name }),
        "remove" => Ok(TeamCmd::RemoveMember { name: member_name }),
        "create" => Ok(TeamCmd::CreateTeam { name: member_name }),
        "delete" => Ok(TeamCmd::DeleteTeam { name: member_name }),
        _ => Err("Unknown command")?,
    }
}

fn create_team(team_name: String) -> Result<(), Box<dyn Error>> {
    let team = team::Team::new(team_name.clone());
    let team_filename = format!("teams/{}.txt", team_name);
    team.save_team(&team_filename)?;
    Ok(())
}

fn delete_team(team_name: String) -> Result<(), Box<dyn Error>> {
    let team_filename = format!("teams/{}.txt", team_name);
    team::delete_team(&team_filename)?;
    Ok(())
}
