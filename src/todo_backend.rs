pub mod todo {
    use chrono::{Duration, NaiveDateTime};
    #[allow(dead_code)]
    use core::fmt;
    use serde::{Deserialize, Serialize};
    use std::{error::Error, io::Result as ResultIo};
    use std::{io::ErrorKind, vec::Vec};
    #[derive(Debug)]
    pub enum TodoError {
        TodoOutOfBoundsError,
        TodoAddingError,
    }

    impl fmt::Display for TodoError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                TodoError::TodoOutOfBoundsError => write!(f, "Input out of bounds"),
                TodoError::TodoAddingError => write!(f, "Adding failed"),
            }
        }
    }
    impl Error for TodoError {}

    #[derive(Serialize, Deserialize)]
    pub struct TodoList {
        pub todo_items: Vec<TodoItem>,
        pub completed_items: Vec<TodoItem>,
    }
    impl TodoList {
        pub const fn new() -> TodoList {
            TodoList {
                todo_items: Vec::new(),
                completed_items: Vec::new(),
            }
        }
        pub fn add_item(&mut self, item_title: &str) -> ResultIo<usize> {
            self.todo_items
                .push(TodoItem::new(item_title.to_string(), None));
            return Ok(self.todo_items.len() - 1);
        }
        pub fn add_item_with_date(&mut self, item_title: &str, date: &str) -> ResultIo<usize> {
            let date = match NaiveDateTime::parse_from_str(date, "%Y %b %d %H:%M:%S") {
                Ok(r) => r,
                Err(_) => return Err(std::io::ErrorKind::Unsupported.into()),
            };

            let time_now = chrono::offset::Local::now();
            if date.signed_duration_since(time_now.naive_local()) < Duration::zero() {
                return Err(std::io::ErrorKind::Unsupported.into());
            }
            self.todo_items
                .push(TodoItem::new(item_title.to_string(), Some(date)));
            return Ok(self.todo_items.len() - 1);
        }

        pub fn complete_item(&mut self, item_id: usize) -> ResultIo<()> {
            if item_id > self.todo_items.len() - 1 {
                return Err(ErrorKind::InvalidInput.into());
            }
            self.todo_items[item_id].completed = true;
            self.completed_items.push(self.todo_items.remove(item_id));
            return Ok(());
        }
        pub fn uncomplete_item(&mut self, item_id: usize) -> ResultIo<()> {
            if item_id > self.completed_items.len() - 1 {
                return Err(ErrorKind::InvalidInput.into());
            }
            self.completed_items[item_id].completed = false;
            self.todo_items.push(self.completed_items.remove(item_id));
            return Ok(());
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
            match self
                .todo_items
                .iter()
                .enumerate()
                .find(|(_index, item)| item.title == item_title)
            {
                Some((id, _)) => Some(id),
                None => None,
            }
        }
        pub fn todo_len(&self) -> usize {
            return self.todo_items.len();
        }
        pub fn completed_len(&self) -> usize {
            return self.completed_items.len();
        }
    }

    #[derive(Serialize, Deserialize)]
    pub struct TodoItem {
        pub title: String,
        pub completed: bool,
        pub due_date: Option<NaiveDateTime>,
    }
    impl TodoItem {
        fn new(item_title: String, due_date: Option<NaiveDateTime>) -> TodoItem {
            TodoItem {
                title: item_title,
                completed: false,
                due_date,
            }
        }
        pub fn print(&self) {
            print!("{item}: ", item = self.title);
            match self.completed {
                false => println!("[]"),
                true => println!("[X]"),
            }
        }
    }
}
