use rusqlite::{Connection,Result};
use std::path::PathBuf;

// Absolute path to the database, located next to the daemon binary.
pub fn db_path() -> PathBuf {
    let mut p = std::env::current_exe().expect("failed to determine executable path");
    p.pop();
    p.push("data.db");
    p
}

//function of create tables
pub fn init_db() -> Result<()> {
    let conn =Connection::open(db_path())?;

//table of information for authentication
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            user_id TEXT PRIMARY KEY,
            email TEXT UNIQUE NOT NULL,
            username TEXT NOT NULL,
            password_hash TEXT NOT NULL,
            is_login INTEGER NOT NULL DEFAULT 0,
            warning INTEGER NOT NULL DEFAULT 0,
            block_apps TEXT
        )",
        [],
    )?;
//table of attempt
    conn.execute(
        "CREATE TABLE IF NOT EXISTS attempt(
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        user_id TEXT NOT NULL,
        Status INTEGER NOT NULL,
        date TEXT NOT NULL,
        week_date TEXT NOT NULL,
        time INTEGER NOT NULL
        )",
        [],
    )?;
//table of daily stastus
    conn.execute(
        "CREATE TABLE IF NOT EXISTS daily_status(
        user_id TEXT NOT NULL,
        date TEXT NOT NULL,
        week_date TEXT NOT NULL,
        totaltime INTEGER,
        PRIMARY KEY(user_id, date)
        )",
        [],
    )?;
    Ok(())    
}