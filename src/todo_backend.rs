pub mod todo{
    use core::fmt;
    use std::error::Error;
    use std::vec::Vec;
    use serde::{
        Serialize,
        Deserialize
    };

    #[derive(Debug)]
    pub enum TodoError {
        TodoOutOfBoundsError,
        TodoAddingError
    }

    impl fmt::Display for TodoError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                TodoError::TodoOutOfBoundsError => write!(f, "Input out of bounds"),
                TodoError::TodoAddingError => write!(f, "Adding failed")
            }
        }
    }
    impl Error for TodoError {}
    


    #[derive(Serialize, Deserialize)]
    pub struct TodoList {
        pub todo_items: Vec<TodoItem>,
        pub completed_items: Vec<TodoItem>,
    } impl TodoList {
        pub fn new() -> TodoList {
            TodoList {
                todo_items: Vec::new(),
                completed_items: Vec::new(),
            }
        }
        pub fn add_item(&mut self, item_title: &str) -> Result<usize, Box<dyn Error>> {
            self.todo_items.push(TodoItem::new(item_title.to_string()));
            return Ok(self.todo_items.len() - 1);
        } 
        pub fn complete_item(&mut self, item_id: usize) -> Result<(), TodoError> {
            if item_id > self.todo_items.len() - 1 {
                return Err(TodoError::TodoOutOfBoundsError);
            }
            self.todo_items[item_id].completed = true;
            self.completed_items.push(self.todo_items.remove(item_id));
            return Ok(())
        } 
        pub fn uncomplete_item(&mut self, item_id: usize) -> Result<(), TodoError> {
            if item_id > self.todo_items.len() - 1 {
                return Err(TodoError::TodoOutOfBoundsError);
            }
            self.completed_items[item_id].completed = false;
            self.todo_items.push(self.completed_items.remove(item_id));
            return Ok(())
        }
        pub fn print_list(&self) {
            self.todo_items
                .iter()
                .enumerate()
                .for_each(|(index, item)| {
                    print!("{index}. ");
                    item.print(); 
                })
        }
        pub fn find_item_id(&self, item_title: &str) -> Option<usize> {
            match self.todo_items
                .iter()
                .enumerate()
                .find(|(index, item)| item.title == item_title) {
                    Some((id, _)) => Some(id),
                    None => None
                }
        }
    }

    #[derive(Serialize, Deserialize)]
    pub struct TodoItem {
        pub title: String,
        pub completed: bool,
    } impl TodoItem {
        fn new(item_title: String) -> TodoItem {
            TodoItem {
                title: item_title,
                completed: false
            }
        }
        pub fn print(&self) {
            print!("{item}: ", item = self.title);
            match self.completed { 
                false => println!("[]"),
                true => println!("[X]")
            }
        }
    }
}
