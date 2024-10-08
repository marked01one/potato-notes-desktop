use rusqlite::{Error, Connection, Result};


#[derive(Debug)]
struct Task {
    guid: String,

}


pub fn create_db() -> Result<()> {
    let conn = Connection::open_in_memory()
        .inspect_err(|e| panic!("Failed to connect to database! {e}"))
        .unwrap();

    let create = "CREATE TABLE Task (
        guid VARCHAR(63) PRIMARY KEY,
        title VARCHAR(255),
        content TEXT,
        time_created DATETIME,
        time_deadline DATETIME
    )";

    conn.execute(create, ())
        .inspect_err(|e| panic!("Failed to create database! {e}"))
        .unwrap();



    return Ok(());
} 