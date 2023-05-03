pub mod data_handler {
    use crate::todo_backend::todo::{TodoList, TodoItem};
    use std::{
        fs::File,
        io::{self, Read, Write}, path::PathBuf,
    };


    pub fn load_todo_list(file: &PathBuf) -> io::Result<TodoList> {
        if !file.exists() {
            generate_file(file)?;
        } 
        let mut opened_file = File::open(file).expect("Failed to Open Save File");

        let mut file_contents = String::new();
        opened_file.read_to_string(&mut file_contents)?;

        let mut todo: TodoList = serde_json::from_str(&file_contents)?;
        todo.completed_items = Vec::new();
        return Ok(todo);
    }

    pub fn save_todo_list(todo_list: &TodoList, path: PathBuf) -> io::Result<()> {
        let serialized_todo = serde_json::to_string(&todo_list)?;
        let mut file = File::create(path)?;
        file.write_all(serialized_todo.as_bytes())?;
        return Ok(());
    }

    fn generate_file(file: &PathBuf) -> io::Result<()> {
        let mut file = File::create(file)?;
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
