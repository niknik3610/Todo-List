pub mod data_handler {
    use crate::todo_backend::todo::TodoList;
    use serde::{Serialize, Deserialize};
    use std::{fs::File, io::Write};

    const DB_PATH: &str = "/data/db.json";
    pub enum DatabaseError {
        ReadError, 
        ParseError
    }
    
    pub fn save_todo_list(todo_list: &TodoList) -> Result<(), DatabaseError> {
        let serialized_todo = serde_json::to_string(&todo_list).unwrap();
        let mut file = File::create("Tester.txt").unwrap();
        file.write_all(serialized_todo.as_bytes());
        return Ok(())
    }
}
