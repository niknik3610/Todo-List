pub mod todo{
    use std::vec::Vec;
    
    #[derive(Debug)]
    pub enum TodoError {
        TodoOutOfBoundsError,
    }

    pub struct TodoList {
        pub items: Vec<TodoItem>,
    } impl TodoList {
        pub fn new() -> TodoList {
            TodoList {
                items: Vec::new(),
            }
        }
        pub fn add_item(&mut self, item_title: &str) -> Result<usize, &str> {
            self.items.push(TodoItem::new(item_title.to_string()));
            return Ok(self.items.len() - 1);
        } 
        pub fn complete_item(&mut self, item_id: usize) -> Result<(), TodoError> {
            if item_id > self.items.len() - 1 {
                return Err(TodoError::TodoOutOfBoundsError);
            }
            self.items[item_id].completed = true;
            return Ok(())
        } 
        pub fn print_list(&self) {
            self.items
                .iter()
                .enumerate()
                .for_each(|(index, item)| {
                    print!("{index}. ");
                    item.print(); 
                })
        }
        pub fn find_item_id(&self, item_title: &str) -> Option<usize> {
            match self.items
                .iter()
                .enumerate()
                .find(|(index, item)| item.title == item_title) {
                    Some((id, _)) => Some(id),
                    None => None
                }
        }
    }

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
