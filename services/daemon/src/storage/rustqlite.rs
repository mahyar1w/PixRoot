use rusqlite::{Connection,Result};

//function of create tables
pub fn init_db() -> Result<()> {
    let conn =Connection::open("data.db")?;


    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            user_id TEXT PRIMARY KEY,
            email TEXT UNIQUE NOT NULL,
            username TEXT NOT NULL,
            password_hash TEXT NOT NULL
        )",
        [],
    )?;
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