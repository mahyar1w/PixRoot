mod storage;

fn main() {
    storage::database::init_db().unwrap();

    
}