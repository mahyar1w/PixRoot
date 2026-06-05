use rusqlite::{Connection,Result};
use chrono::{Local,Datelike};


//function of add attempt
pub fn add_attempt(
    conn: &Connection,
    status: bool,
    week_date: &str,
    date: &str,
    time: i32,
)-> Result<()>{
    conn.execute("
        INSERT INTO attempt(status,date,week_date,time)
        VALUES(?1,?2,?3,?4)",
        (status,date,week_date,time)
        )?;
    println!("attempt added");
    Ok(())
}

//function of add attempt to daily status
pub fn update_daily_status(
    conn: &Connection,
    status: bool,
    date: &str,
    week_date: &str,
    time: i32,
)-> Result<()>{
    if !status{
        return Ok(())
    }
    conn.execute("
    INSERT INTO Daily_status(date,week_date, totaltime)
        VALUES(?1, ?2, ?3)
        ON CONFLICT(date)
        DO UPDATE SET
        totaltime = totaltime + excluded.totaltime",
        (date,week_date,time),
        )?;
    println!("attempt add to daily table");
    Ok(())
}

//function of call other function
pub fn register_attempt(status: bool,time : i32) ->Result<()>{
    //connect to db file
    const DB_PATCH : &str = "data.db";
    let conn = Connection::open(DB_PATCH)?;
    //get date information
    let date = chrono::Local::now().format("%Y-%m-%d").to_string();
    let week_date = format!("{:?}", Local::now().weekday());
    //call other function
    add_attempt(&conn, status,&week_date, &date, time)?;
    update_daily_status(&conn, status,&date,&week_date, time)?;
    Ok(())
}

