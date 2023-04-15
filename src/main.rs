use crate::todo_backend::todo::TodoList;
use crate::data_handler::data_handler::*;
mod todo_backend;
mod data_handler;
mod tui_handler;

fn main() {    
    let mut list = load_todo_list().unwrap();
    tui_handler::tui_handler::run_tui(&mut list).unwrap();
    data_handler::data_handler::save_todo_list(&list).unwrap();
}
