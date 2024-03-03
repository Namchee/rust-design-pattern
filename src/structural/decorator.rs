// Decorator is a pattern that allows you to attach new behavior
// to an object using a wrapper object without modifying the original object
//
// In this example, we have a data repository that fetches data from db. Currently,
// the app is running well on production. After a few months, you realized
// that the app is a bit slow, so you want to determine the actual query time
// since you're not sure if your query is slow or the bottleneck is somewhere else.
// As this is a benchmark code that won't live long, you don't want to modify the original methods.
// A decorator that benchmarks the method can used to achieve this.

use std::time::Instant;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub age: u16,
}
pub trait UserRepository {
    fn find_user(&mut self, name: String) -> Option<User>;
}

#[derive(Clone)]
pub struct UserPostgreRepository {}
impl UserRepository for UserPostgreRepository {
    fn find_user(&mut self, _: String) -> Option<User> {
        // imagine if we query the db
        Some(User { name: "John Doe".to_string(), age: 37 })
    }
}

#[allow(dead_code)]
pub struct UserRepositoryWithLogger {
    pub repo: Box<dyn UserRepository>,
}

impl UserRepository for UserRepositoryWithLogger {
    fn find_user(&mut self, name: String) -> Option<User> {
        println!("Querying DB for user {}", name);

        let start = Instant::now();

        let result = self.repo.find_user(name.clone());

        println!("Finish querying DB for user {} in {}s", name, start.elapsed().as_secs());

        result
    }
}
