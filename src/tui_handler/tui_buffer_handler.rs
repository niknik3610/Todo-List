use crate::todo_backend::todo::TodoList;  

use std::io::{ErrorKind, self};
use super::tui_handler::{State, AddState};

pub fn submit_buffer(
    current_state_data: &State,
    output_buffer: &str,
    date_storage_buff: &String,
    todo: &mut TodoList,
    ) -> io::Result<()> { 
    if let State::AddingTodo(state) = *current_state_data { 
        match state {
            AddState::EnteringName => todo.add_item(&output_buffer)?,
            AddState::EnteringDate(_) => { 
                // todo.add_item(&*format!("'{date_storage_buff}'"))?;
                todo.add_item_with_date(&output_buffer, &*date_storage_buff)? 
            }
        };
        return Ok(());
    }

    let output_buffer = match output_buffer.parse::<usize>() {
        Ok(r) => r,
        Err(_) => return Err(ErrorKind::InvalidInput.into())
    }; 

    match *current_state_data {
        State::CompletingTodo => {
            if output_buffer > todo.todo_len() - 1 {
                return Err(ErrorKind::InvalidInput.into());
            }
            todo.complete_item(output_buffer)?;
        },
        State::UncompletingTodo => {
            if output_buffer > todo.completed_len() - 1 {
                return Err(ErrorKind::InvalidInput.into());
            }
            todo.uncomplete_item(output_buffer)?;
        }
        _ => {}
    }

    return Ok(());
}

pub fn swap_buffers(prev_buff: &str, storage_buff: &mut String) -> io::Result<()> {
    *storage_buff = prev_buff.to_string();
    return Ok(())
}
