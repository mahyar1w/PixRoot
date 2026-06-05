use rusqlite::{Connection,Result};

//function of create tables
pub fn init_db() -> Result<()> {
    let conn =Connection::open("data.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS attempt(
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        Status INTEGER NOT NULL,
        date TEXT NOT NULL,
        week_date TEXT NOT NULL,
        time INTEGER NOT NULL

        )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Daily_status(
        date TEXT PRIMARY KEY,
        week_date TEXT NOT NULL,
        totaltime INTEGER    
        )",
        [],


    )?;



    Ok(())

    
}