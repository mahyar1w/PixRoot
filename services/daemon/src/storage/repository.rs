use rusqlite::Connection;
use chrono::{Local,Datelike};
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
//function for check email in db
pub fn check_email(
    conn: &Connection,
    email: &str,
)->rusqlite::Result<bool>{

    let mut stmt = conn.prepare(
        "SELECT 1 FROM users WHERE email = ?1"
    )?;

    let mut rows = stmt.query([email])?;

    if rows.next()?.is_some() {
    Ok(true)
    } 
    else {
    Ok(false)
    }
}
//function for sign up
pub fn sign_up(
    email: &str,
    username: &str,
    password: &str,
)-> Result<(), Box<dyn std::error::Error>>{

    let conn = Connection::open("data.db")?;
    let user_id = Uuid::new_v4().to_string();
    let password_hash=hasher_password(password)?;

    conn.execute(
        "
        INSERT INTO users(
            user_id,
            email,
            username,
            password_hash
        )
        VALUES(?1, ?2, ?3, ?4)
        ",
        (
            &user_id,
            email,
            username,
            password_hash,
        ),
    )?;
    println!("user created");

    Ok(())

}
pub enum LoginState {
    NeedSignup,
    WrongPassword,
    Success(LoginResult),
}
pub struct LoginResult {
    pub user_id: String,
    pub username: String,
}
//function of check password and login
pub fn check_password(
    conn: &Connection,
    email: &str,
    password: &str,
) -> Result<LoginState, Box<dyn std::error::Error>> {

    let mut stmt = conn.prepare(
        "SELECT user_id, username, password_hash FROM users WHERE email = ?1"
    )?;

    let mut rows = stmt.query([email])?;

    let row = match rows.next()? {
        Some(row) => row,
        None => return Ok(LoginState::NeedSignup),
    };

    let user_id: String = row.get(0)?;
    let username: String = row.get(1)?;
    let db_password: String = row.get(2)?;

    if verify_password(password, &db_password)? {
        Ok(LoginState::Success(LoginResult {
            user_id,
            username,
        }))
    } else {
        Ok(LoginState::WrongPassword)
    }
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
    user_id: &str,
    status: bool,
    time: i32
) -> rusqlite::Result<()> {

    let conn = Connection::open("data.db")?;

    let date = chrono::Local::now().format("%Y-%m-%d").to_string();
    let week_date = format!("{:?}", chrono::Local::now().weekday());

    add_attempt(&conn, user_id, status, &week_date, &date, time)?;
    update_daily_status(&conn, user_id, status, &date, &week_date, time)?;

    Ok(())
}

