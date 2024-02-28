// Singleton pattern is a design pattern that ensures an object is only
// created once in the application.
//
// Generally, it's best to use a connection pool to manage database connections
// but I will use Database as an example here.
//
// Rust doesn't support mutable global variable in safe code. We need to run another
// package called lazy_static 

use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref DB: Mutex<Database> = Mutex::new(Database{});
}

struct Database {}

impl Database {
    fn exec(&self, query: String) {
        println!("Inserting new rows by executing {}", query);
    }
}

pub fn exec(query: String) {
    DB.lock().unwrap().exec(query);
}
