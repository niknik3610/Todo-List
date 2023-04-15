pub mod data_handler {
    use crate::todo_backend::todo::TodoList;
    use serde::{Serialize, Deserialize};
    use std::{fs::File, io::{Write, Read}, error::Error};

    const DB_PATH: &str = "Tester.txt"; 
    
    pub fn save_todo_list(todo_list: &TodoList) -> Result<(), Box<dyn Error>> {
        let serialized_todo = serde_json::to_string(&todo_list).unwrap();
        let mut file = File::create("Tester.txt").unwrap();
        file.write_all(serialized_todo.as_bytes())?;
        return Ok(())
    }

    pub fn load_todo_list() -> Result<TodoList, Box<dyn Error>> {
        let mut file = File::open(DB_PATH)?;
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents)?;

        let todo = serde_json::from_str(&file_contents)?;
        return Ok(todo);
    }
}
