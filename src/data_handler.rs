pub mod data_handler {
    use crate::todo_backend::todo::TodoList;
    use std::{fs::File, io::{Write, Read, self}, error::Error};

    const DB_PATH: &str = "Todo_Data"; 

    pub fn load_todo_list() -> Result<TodoList, io::Error> {
        let mut file = File::open(DB_PATH)?;
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents)?;

        let todo = serde_json::from_str(&file_contents)?;
        return Ok(todo);
    }

    pub fn save_todo_list(todo_list: &TodoList) -> Result<(), io::Error> {
        let serialized_todo = serde_json::to_string(&todo_list)?;
        let mut file = File::create(DB_PATH)?;
        file.write_all(serialized_todo.as_bytes())?;
        return Ok(())
    }

    pub fn handle_opening_errors(e: io::Error) -> Result<(), ()> {
        match e.kind() {
            io::ErrorKind::NotFound => {
                let mut file =  File::create(DB_PATH).unwrap();
                file.write_all(b"{\"todo_items\":[],\"completed_items\":[]}"); 
                println!("Todo file generated"); 
                return Ok(());
            }
            io::ErrorKind::PermissionDenied => {
                println!("App does not have permission to access save file");
                return Err(());
            }
            _ => return Err(())
        }
    }

}
