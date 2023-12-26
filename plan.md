# scrumMD - A CLI tool for managing your scrum meetings using markdown

## Goal of this project

The goal of this project is to create a CLI tool that helps you manage your scrum meetings. This will be in the form of a CLI tool that will allow you to manage a team and create scrum boards and manage them. Using this tools you can create action items, move them around, add members. You can also construct meeting agendas.

## Features

- Create a team
  - Add members to the team
  - Remove members from the team
- Create a scrum board
  - Add action items to the scrum board
  - Move action items around the board
  - Assign action items to members
- Create a meeting agenda
	- Add action items to the agenda
	- Add members to the agenda
	- Add notes to the agenda
	- Add a meeting summary to the agenda

## How to use

### team functionality

to add a team member to a team
```bash
scrum team add <member-name> [--team_name <team-name>]
```

To remove a team member from a team
```bash
scrum team remove <member-name> [--team_name <team-name>]
```
