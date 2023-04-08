pub mod todo{
    use std::collections::HashMap;
    pub struct TodoList {
        next_id: usize,
        items: HashMap<usize, TodoItem>,
    } impl TodoList {
        pub fn new() -> TodoList {
            TodoList {
                next_id: 1,
                items: HashMap::<usize, TodoItem>::new(),
            }
        }
        pub fn add_item(&mut self, item_title: &str) -> Result<usize, &str> {
            self.items.insert(self.next_id, TodoItem::new(item_title.to_string()));
            self.next_id += 1;
            return Ok(self.next_id - 1);
        } 
        pub fn complete_item(&mut self, item_id: usize) -> Result<(), &str> {
            match self.items.get_mut(&item_id) {
                None => return Err("Item doesn't Exist"),
                Some(result) => result.completed = true 
            } 
            return Ok(())
        } 
        pub fn print_list(&self) {
            self.items
                .iter()
                .for_each(|(id, item)| {
                    print!("{id}. ");
                    item.print(); 
                })
        }
        pub fn find_item_id(&self, item_title: &str) -> Option<usize> {
            match self.items
                .iter()
                .find(|(_, item)| item.title == item_title) {
                    Some((id, _)) => Some(*id),
                    None => None
                }
        }
    }

    struct TodoItem {
        title: String,
        completed: bool,
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
