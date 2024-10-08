use std::any::{Any, TypeId};

use rusqlite::{Error, Connection, Result};
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Task {
    guid: String,
    title: String,
    content: String,
    time_created: DateTime<Utc>,
    time_deadline: DateTime<Utc>
}


pub fn create_task_table(conn: &Connection) -> Result<i32, Error> {
    let query = "CREATE TABLE Tasks (
        guid VARCHAR(63) PRIMARY KEY,
        title VARCHAR(255),
        content TEXT,
        time_created DATETIME,
        time_deadline DATETIME
    )";

    let result = conn.execute(query, ());

    if result.is_err() { return Err(result.err().unwrap()) }

    return Ok(0);
}

pub fn get_task_by_id(conn: &Connection, id: &str) -> Result<usize, Error> {
    return Ok(0);
}

/// Query to add task to the existing DB connection
pub fn insert_task(conn: &Connection, task: &Task) -> Result<usize, Error> {
    let query = r"INSERT INTO Tasks 
        (guid, title, content, time_created, time_deadline)
        VALUES (?1, ?2, ?3, ?4, ?5)
    ";

    let result = conn.execute(query,(
        &task.guid, 
        &task.title, 
        &task.content, 
        format!("{}", &task.time_created.format("YYYY-MM-DD HH:MM:SS")), 
        format!("{}", &task.time_deadline.format("YYYY-MM-DD HH:MM:SS")) 
    ));

    return result;
}


pub fn insert_tasks(conn: &mut Connection, tasks: &Vec<Task>) -> Result<usize, Error> {
    let query = r"INSERT INTO Tasks 
        (guid, title, content, time_created, time_deadline)
        VALUES (?1, ?2, ?3, ?4, ?5)
    ";

    let tx = conn.transaction();
    if tx.is_err() {
        eprintln!("Failed to create insert transaction! {}", tx.as_ref().unwrap_err());
        return Err(tx.unwrap_err());
    }

    let trans = tx.unwrap();

    for tk in tasks {
        let result = trans.execute(query, (
            &tk.guid,
            &tk.title,
            &tk.content,
            format!("{}", &tk.time_created.format("YYYY-MM-DD HH:MM:SS")), 
            format!("{}", &tk.time_deadline.format("YYYY-MM-DD HH:MM:SS"))
        ));

        if result.is_err() {
            eprintln!("Failed to execute this transaction! {}", result.as_ref().unwrap_err());
            return result;
        }
    }
    
    let commit_result = trans.commit();
    
    if commit_result.is_err() {
        eprintln!("Failed to commit this transaction to connection! {}", commit_result.as_ref().unwrap_err());
        return Err(commit_result.unwrap_err());
    }

    return Ok(tasks.len());
}



pub fn delete_task_by_id(conn: &Connection, guid: &str) -> Result<usize, Error> {
    let query = "DELETE FROM Tasks WHERE guid = ?1";

    let result = conn.execute(query, [guid]);

    match &result {
        Err(e) => eprintln!("Failed to delete item! {}", e.to_string()),
        Ok(rows) => println!("Successfully deleted {rows} rows!")
    }

    return result;
}

