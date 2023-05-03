use home::home_dir;
use crate::data_handler::data_handler::*;
mod data_handler;
mod todo_backend;
mod tui_handler;
mod parsing_handler;

fn main() {
<<<<<<< HEAD
    env::set_var("RUST_BACKTRACE", "1");
    
=======
    // let parsed = parsing_handler::parse("").unwrap();
    // println!("{:?}", parsed);
    let mut file = home_dir().expect("Could not find home directory");       
    file.push(".todo_items");
>>>>>>> 6a76d04 (now saves the todolist file into the home directory under .todo_items)
    //error handling the load file
    let mut list = load_todo_list(&file).unwrap();
    tui_handler::tui_handler::run_tui(&mut list).unwrap();
<<<<<<< HEAD
    
    
    save_todo_list(&list).unwrap();
=======

    //For testing purposes not saving currently, uncomment to enable saving list
    save_todo_list(&list, file).unwrap();
>>>>>>> 6a76d04 (now saves the todolist file into the home directory under .todo_items)
}
