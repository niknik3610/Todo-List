use crate::todo_backend::todo::TodoList;
use crate::parsing_handler::{
    parse,
    handle_command,
};

use super::{
    tui_handler::{
        generate_todo,
        AddState,
        BufferAction,
        DateState,
        State
    },
    tui_rendering_handler::TodoItems, 
};
use std::io::{self, ErrorKind};

pub fn submit_buffer(
    current_state_data: &State,
    output_buffer: &str,
    date_storage_buff: &String,
    todo: &mut TodoList,
) -> io::Result<()> {
    match *current_state_data {
        State::AddingTodo => {
            todo.add_item(output_buffer)?;
        },
        State::AddingTodoDate(state) => {
            match state {
                AddState::EnteringName => todo.add_item(&output_buffer)?,
                AddState::EnteringDate(_) => {
                    // todo.add_item(&*format!("'{date_storage_buff}'"))?;
                    todo.add_item_with_date(&output_buffer, &*date_storage_buff)?
                }
            };
        }
        _ => return Err(ErrorKind::InvalidInput.into())

    }
    return Ok(());
}

pub fn manipulate_buffer(
    current_state: &mut State,
    action: BufferAction,
    user_input_buffer: &mut String,
    name_storage_buff: &mut String,
    date_storage_buff: &mut String,
    todo: &mut TodoList,
    todo_items: &mut TodoItems,
) -> io::Result<()> {
    match action {
        BufferAction::Input(input) => user_input_buffer.push(input),
        BufferAction::Backspace => {
            user_input_buffer.pop();
        }
        BufferAction::ExitBuffer => {
            *current_state = State::Viewing;
            *date_storage_buff = String::new();
            *user_input_buffer = String::new();
            *date_storage_buff = String::new();
        }
        BufferAction::SubmitBuffer => {
            match_buffer_submit(
                &mut *current_state,
                user_input_buffer,
                name_storage_buff,
                date_storage_buff,
                todo,
                todo_items,
            )?;
        }
    }
    return Ok(());
}

fn match_buffer_submit(
    current_state: &mut State,
    user_input_buffer: &mut String,
    name_storage_buff: &mut String,
    date_storage_buff: &mut String,
    todo: &mut TodoList,
    todo_items: &mut TodoItems,
) -> io::Result<()> {     
    match *current_state {
        State::AddingTodo => {
            submit_buffer(&current_state, &user_input_buffer, date_storage_buff, todo)?;
            *todo_items = generate_todo(todo);
            *current_state = State::Viewing;
            *user_input_buffer = String::new();
        }
        State::AddingTodoDate(AddState::EnteringName) => {
            swap_buffers(&user_input_buffer, &mut *name_storage_buff)?;
            *user_input_buffer = String::new();
            *current_state = State::AddingTodoDate(AddState::EnteringDate(DateState::Year));
        }
        State::AddingTodoDate(AddState::EnteringDate(state)) => {
            if let DateState::Time = state {
                *date_storage_buff += &*(user_input_buffer);
                submit_buffer(
                    &current_state,
                    &*name_storage_buff,
                    &date_storage_buff,
                    todo,
                )?;

                *current_state = State::Viewing;
                *todo_items = generate_todo(todo);
                *date_storage_buff = String::new();
                *user_input_buffer = String::new();
                *date_storage_buff = String::new();
            } else {
                *date_storage_buff += &*(user_input_buffer.to_owned() + " ");
                *current_state = State::AddingTodoDate(AddState::EnteringDate(state.next().unwrap()));
                *user_input_buffer = String::new();
            }
        }
        //commands go here (will probably move this out at some point
        _ => {
            submit_command(&mut *current_state, &user_input_buffer, todo)?;
            *todo_items = generate_todo(todo);
            *user_input_buffer = String::from("");
        }
    }
    return Ok(());
}

pub fn submit_command(
    current_state: &mut State,
    output_buffer: &str,
    todo: &mut TodoList,
) -> io::Result<()> {
    if let State::EnteringCommand = current_state {
        let parsed = parse(output_buffer)?; 
        *current_state = handle_command(parsed)?;
        return Ok(());
    }

    let output_buffer = match output_buffer.parse::<usize>() {
        Ok(r) => r,
        Err(_) => return Err(ErrorKind::InvalidInput.into()),
    };

    match *current_state {
        State::CompletingTodo => {
            todo.complete_item(output_buffer)?;
        }
        State::UncompletingTodo => {
            todo.uncomplete_item(output_buffer)?;
        } 
        _ => {}
    }

    *current_state = State::Viewing;
    return Ok(());
}
pub fn swap_buffers(prev_buff: &str, storage_buff: &mut String) -> io::Result<()> {
    *storage_buff = prev_buff.to_string();
    return Ok(());
}
