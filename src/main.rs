use crate::todo_backend::todo::TodoList;
mod todo_backend;
mod data_handler;
mod tui_handler;

fn main() {    
    let mut list = TodoList::new();
    list.add_item("Write todo list").unwrap();
    list.add_item("Your Mom").unwrap();
    list.complete_item(0).unwrap();

    tui_handler::tui_handler::run_tui(&mut list).unwrap();
    data_handler::data_handler::save_todo_list(&list);
}
