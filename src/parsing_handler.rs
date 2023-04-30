use std::{
    io::{self, ErrorKind},
    collections::HashMap
};
use phf::phf_map;
use crate::tui_handler::tui_handler::{State, AddState};

#[derive(Clone, Copy, Debug)]
pub enum UserCommand {
    AddTask,
    AddTaskDate,
    CompleteTask,
    UncompleteTask,
    Quit,
}

#[derive(Debug)]
pub struct Command {
    command: UserCommand,
    index: Option<usize>,
}

static MAPPED_COMMANDS: phf::Map<&'static str, UserCommand> = phf_map! {
    "AddTask" => UserCommand::AddTask,
    "AddTaskDate" => UserCommand::AddTaskDate,
    "CompleteTask" => UserCommand::CompleteTask, 
    "UncompleteTask" => UserCommand::UncompleteTask,
    "Quit" => UserCommand::Quit,
};

pub fn parse(user_input: &str) -> io::Result<Command> {
    let tokens = tokenize(user_input);
    let command = match MAPPED_COMMANDS.get(tokens[0]) {
        Some(r) => r,
        None => return Err(ErrorKind::NotFound.into())
    };
    
    let index; 
    if tokens.len() > 1 {
        let parsed_num = match tokens[1].parse::<usize>(){
            Ok(r) => r,
            Err(_) => return Err(ErrorKind::InvalidInput.into())
        };
        index = Some(parsed_num); 
    }
    else {
        index = None;
    }

    let command = Command{command: command.clone(), index};
    return Ok(command);
}

fn tokenize(user_input: &str) -> Vec<&str> {
    let tokens: Vec<&str> = user_input.split(" ").collect();
    return tokens;
} 

pub fn handle_command(cmd: Command) -> io::Result<State> {
    use UserCommand::*;
    match cmd.command {
        AddTask => Ok(State::AddingTodo),
        AddTaskDate => Ok(State::AddingTodoDate(AddState::EnteringName)),
        CompleteTask => Ok(State::CompletingTodo),
        UncompleteTask => Ok(State::UncompletingTodo),
        Quit => Ok(State::Quitting),
    }
}
