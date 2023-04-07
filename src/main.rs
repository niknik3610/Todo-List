use std::collections::HashMap;

pub struct TodoList {
    next_id: usize,
    items: HashMap<usize, TodoItem>,
} impl TodoList {
    pub fn new() -> TodoList {
        TodoList {
            next_id: 0,
            items: HashMap::<usize, TodoItem>::new(),
        }
    }
    pub fn add_item(&mut self, item_title: &str) -> Result<usize, &str> {
        match self.items
            .iter()
            .find(|(_, item)| item.title == item_title) {
                None => {
                    self.items.insert(self.next_id, TodoItem::new(item_title.to_string()));
                    self.next_id += 1;
                    return Ok(self.next_id - 1);
                },
                Some(_) => return Err("This item already exists") 
            } 
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
            .for_each(|(_, item)| item.print())
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

fn main() {
    let mut list = TodoList::new();
    let id_zero = list.add_item("Write todo list").unwrap();
    list.print_list();

    list.complete_item(id_zero).unwrap();
    list.print_list();
}
