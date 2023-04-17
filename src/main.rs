use std::process::exit;
use std::env;
use crate::data_handler::data_handler::*;
mod todo_backend;
mod data_handler;
mod tui_handler;

fn main() {     
    //error handling the load file
   
    env::set_var("RUST_BACKTRACE", "1");

    //get rid of this probably
    let mut list = load_todo_list().unwrap_or_else(|e| 
        match handle_data_errors(e) {
            Ok(()) => load_todo_list().unwrap(),
            Err(e) => {
                eprintln!("{:?}", e);
                exit(1);
            }
        }
    );

    tui_handler::tui_handler::run_tui(&mut list).unwrap();

    //For testing purposes not saving currently, uncomment to enable saving list
    //save_todo_list(&list).unwrap();
}
