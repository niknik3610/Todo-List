pub mod data_handler {
    use crate::todo_backend::todo::TodoList;
    use std::{
        fs::File,
        io::{self, Read, Write},
    };

    const DB_PATH: &str = "Todo_Data";

    pub fn load_todo_list() -> io::Result<TodoList> {
        let mut file = File::open(DB_PATH).unwrap_or_else(|_|{
            generate_file().expect("Failed to Create Save File");
            File::open(DB_PATH).expect("Failed to Open Save File")
        });

        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents)?;

        let todo = serde_json::from_str(&file_contents)?;
        return Ok(todo);
    }

    pub fn save_todo_list(todo_list: &TodoList) -> io::Result<()> {
        let serialized_todo = serde_json::to_string(&todo_list)?;
        let mut file = File::create(DB_PATH)?;
        file.write_all(serialized_todo.as_bytes())?;
        return Ok(());
    }

    fn generate_file() -> io::Result<()> {
        let mut file = File::create(DB_PATH)?;
        match file.write_all(b"{\"todo_items\":[],\"completed_items\":[]}") {
            Ok(_) => return Ok(()),
            Err(e) => return Err(e),
        }
    }
    
    pub fn handle_data_errors(e: io::Error) -> io::Result<()> {
        match e.kind() {
            io::ErrorKind::PermissionDenied => {
                println!("App does not have permission to access save file");
                return Err(e);
            }
            _ => return Err(e),
        }
    }
}
