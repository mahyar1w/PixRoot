use rusqlite::{Connection, Result};
use crate::storage::rustqlite::db_path;
use chrono::{NaiveDate,Datelike,Duration};
use uuid::Uuid;
use argon2::{Argon2,PasswordHasher,password_hash::{SaltString,PasswordHash,PasswordVerifier,rand_core::OsRng,},};

//functions of hash
pub fn hasher_password(
    password: &str
) -> std::result::Result<String, Box<dyn std::error::Error>> {

    let salt = SaltString::generate(&mut OsRng);

    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| format!("hash error: {e}"))?
        .to_string();

    Ok(hash)
}
pub fn verify_password(
    password: &str,
    stored_hash: &str,
) -> std::result::Result<bool, Box<dyn std::error::Error>> {

    let parsed_hash = PasswordHash::new(stored_hash)
        .map_err(|e| format!("parse hash error: {e}"))?;

    Ok(
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    )
}
// logout functions 
pub fn logout(
    conn: &Connection,
    user_id: &str,
) -> Result<()> {

    conn.execute(
        "
        UPDATE users
        SET is_login = 0
        WHERE user_id = ?1
        ",
        [user_id],
    )?;

    Ok(())
}
pub struct LoggedUser {
    pub user_id: String,
    pub username: String,
}
// Functions of log_in without pass
pub fn get_logged_in_user(
    conn: &Connection,
) -> Result<Option<LoggedUser>> {

    let mut stmt = conn.prepare(
        "
        SELECT user_id, username
        FROM users
        WHERE is_login = 1
        LIMIT 1
        "
    )?;

    let mut rows = stmt.query([])?;

    if let Some(row) = rows.next()? {

        return Ok(Some(LoggedUser {
            user_id: row.get(0)?,
            username: row.get(1)?,
        }));
    }

    Ok(None)
}

pub struct UserData {
    pub user_id: String,
    pub username: String,
    pub password_hash: String,
}
//function for check email in db
pub fn check_email(
    conn: &Connection,
    email: &str,
) -> Result<Option<UserData>> {

    let mut stmt = conn.prepare(
        "
        SELECT user_id, username, password_hash
        FROM users
        WHERE email = ?1
        "
    )?;

    let mut rows = stmt.query([email])?;

    if let Some(row) = rows.next()? {

        return Ok(Some(UserData {
            user_id: row.get(0)?,
            username: row.get(1)?,
            password_hash: row.get(2)?,
        }));
    }

    Ok(None)
}
//functions of check pass and login
pub fn check_password(
    user: &UserData,
    password: &str,
) -> std::result::Result<bool, Box<dyn std::error::Error>> {

    verify_password(
        password,
        &user.password_hash,
    )
}
//set login=1
pub fn set_login(
    conn: &Connection,
    user_id: &str,
) -> rusqlite::Result<()> {

    conn.execute(
        "
        UPDATE users
        SET is_login = 1
        WHERE user_id = ?1
        ",
        [user_id],
    )?;

    Ok(())
}
//function for sign up
pub fn sign_up(
    email: &str,
    username: &str,
    password: &str,
)-> std::result::Result<(), Box<dyn std::error::Error>>{

    let conn = Connection::open(db_path())?;
    let user_id = Uuid::new_v4().to_string();
    let password_hash=hasher_password(password)?;

    conn.execute(
        "
        INSERT INTO users(
            user_id,
            email,
            username,
            password_hash,
            is_login,
            warning,
            block_apps
        )
        VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7)
        ",
        (
            &user_id,
            email,
            username,
            password_hash,
            1,
            0,
            "",
        ),
    )?;
    println!("user created");

    Ok(())

}
//function of add attempt
pub fn add_attempt(
    conn: &Connection,
    user_id: &str,
    status: bool,
    week_date: &str,
    date: &str,
    time: i32,
) -> rusqlite::Result<()> {

    conn.execute(
        "INSERT INTO attempt(user_id, status, date, week_date, time)
         VALUES(?1, ?2, ?3, ?4, ?5)",
        (user_id, status, date, week_date, time),
    )?;

    Ok(())
}

//function of add attempt to daily status
pub fn update_daily_status(
    conn: &Connection,
    user_id: &str,
    status: bool,
    date: &str,
    week_date: &str,
    time: i32,
) -> rusqlite::Result<()> {

    if !status {
        return Ok(());
    }

    conn.execute(
        "INSERT INTO daily_status(user_id, date, week_date, totaltime)
         VALUES(?1, ?2, ?3, ?4)
         ON CONFLICT(user_id, date)
         DO UPDATE SET
         totaltime = totaltime + excluded.totaltime",
        (user_id, date, week_date, time),
    )?;

    Ok(())
}

//function of call other function
pub fn register_attempt(
    conn: &Connection,
    user_id: &str,
    status: bool,
    time: i32,
) -> rusqlite::Result<()> {
    println!("start save session in database");
    let date =chrono::Local::now().format("%Y-%m-%d").to_string();
    let week_date =format!("{:?}",chrono::Local::now().weekday());

    add_attempt(conn,user_id,status,&week_date,&date,time,)?;
    update_daily_status(conn,user_id,status,&date,&week_date,time,)?;
    println!("session saved success");
     Ok(())
}

// set blocked apps (comma joined string) for the logged in user
pub fn set_blocked_apps(
    conn: &Connection,
    blocked_apps: &str,
) -> rusqlite::Result<()> {
    conn.execute(
        "UPDATE users SET block_apps = ?1 WHERE is_login = 1",
        [blocked_apps],
    )?;
    Ok(())
}

// get blocked apps (comma joined string) for the logged in user
pub fn get_blocked_apps(
    conn: &Connection,
) -> rusqlite::Result<String> {
    let result: Result<String> = conn.query_row(
        "SELECT block_apps FROM users WHERE is_login = 1 LIMIT 1",
        [],
        |row| row.get(0),
    );
    Ok(result.unwrap_or_default())
}

// set warning flag for the logged in user
pub fn set_warning(
    conn: &Connection,
    warning: bool,
) -> rusqlite::Result<()> {
    conn.execute(
        "UPDATE users SET warning = ?1 WHERE is_login = 1",
        [warning as i32],
    )?;
    Ok(())
}
//get total time focus for a day
pub fn get_total_day_time(
    conn: &Connection,
    user_id: &str,
) -> Result<i32> {
    println!("getting total time from daily_status");

    let date = chrono::Local::now()
        .format("%Y-%m-%d")
        .to_string();

    let total: i32 = conn.query_row(
        "
        SELECT COALESCE(totaltime, 0)
        FROM daily_status
        WHERE user_id = ?1
        AND date = ?2
        LIMIT 1
        ",
        [user_id, &date],
        |row| row.get(0),
    )?;

    Ok(total)
}

// get warning flag for the logged in user
pub fn get_warning(
    conn: &Connection,
) -> rusqlite::Result<bool> {
    let result: Result<i32> = conn.query_row(
        "SELECT warning FROM users WHERE is_login = 1 LIMIT 1",
        [],
        |row| row.get(0),
    );
    Ok(result.unwrap_or(0) != 0)
}

pub struct DayStat {
    pub date: String,
    pub week_day: String,
    pub minutes: i32,
}
pub fn get_week_stats(
    conn: &Connection,
    user_id: &str,
    end_date: &str,
)->rusqlite::Result<Vec<DayStat>>{
    let end_date = NaiveDate::parse_from_str(end_date,"%Y-%m-%d").unwrap();
    let mut stats = Vec::new();
    for i in (0..7).rev() {

        let current_date =
            end_date - Duration::days(i);

        let date_str =
            current_date.format("%Y-%m-%d").to_string();
        let minutes = conn.query_row(
        "SELECT totaltime
        FROM daily_status
        WHERE user_id = ?1
        AND date = ?2",
        [user_id, &date_str],|row| row.get::<_, i32>(0),).unwrap_or(0);

        stats.push(DayStat {
        date: date_str, week_day: format!("{:?}", current_date.weekday()),minutes,
        });
    }
     Ok(stats)
}
