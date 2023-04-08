pub mod data_handler {
    const DB_PATH: &str = "/data/db.json";
    pub enum DatabaseError {
        ReadError, 
        ParseError
    }

}
