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

to add a team member to a team:
```bash
scrum team add <member-name> [--team_name <team-name>]
```

To remove a team member from a team:
```bash
scrum team remove <member-name> [--team_name <team-name>]
```

To create a team:
```bash
scrum team create <team-name>
```

To delete a team:
```bash
scrum team delete <team-name>
```

### scrum board functionality

To create a scrum board:
```bash
scrum board create <board-name> [--team_name <team-name>]
```

To remove a scrum board:
```bash
scrum board remove <board-name> [--team_name <team-name>]
```

To add an action item to a scrum board:
```bash
scrum board add <action-item-name> [--board_name <board-name> --member_name <member-name>]
```

To remove an action item from a scrum board:
```bash
scrum board remove <action-item-name> [--board_name <board-name>]
```

To assign an action item to a team member:
```bash
scrum board assign <action-item-name> <member-name> [--board_name <board-name>]
```

To move an action item to a different column:
```bash
scrum board move <action-item-name> <column-name> [--board_name <board-name>]
```

## Known bugs

### team functionality

[x] If you create a team that already exists, it overwrites the old team with a new blank one.
[x] If you add a team member that already exists, it removes all members from the team.
[x] If you remove a team member that doesn't exist, it removes all members from the team.