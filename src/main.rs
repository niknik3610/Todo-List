use crate::todo_backend::todo::TodoList;
mod todo_backend;
mod data_handler;
mod tui_handler;

fn main() {
    /*
    let mut list = TodoList::new();
    let id_zero = list.add_item("Write todo list").unwrap();
    list.print_list();

    list.complete_item(id_zero).unwrap();
    list.print_list();
    */
    tui_handler::tui_handler::run_tui().unwrap();
}
