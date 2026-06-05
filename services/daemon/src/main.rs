mod storage;

fn main() {
    storage::rustqlite::init_db().unwrap();

    
}