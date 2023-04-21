use std::env;

use crate::data_handler::data_handler::*;
mod data_handler;
mod todo_backend;
mod tui_handler;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    //error handling the load file
    let mut list = load_todo_list().unwrap(); 
    tui_handler::tui_handler::run_tui(&mut list).unwrap();

    //For testing purposes not saving currently, uncomment to enable saving list
    //save_todo_list(&list).unwrap();
}
