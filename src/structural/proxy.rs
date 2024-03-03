// Proxy is a design pattern that allows substitute to an object
// Proxy controls the access to the original object. Hence, allowing
// you to perform something before or after the routine.
//
// In this example, we have a data repository that fetches data from db. Currently,
// the app is running well on production. After a few months, you realized
// that the app is a bit slow, so you want to add caching. However, the code 
// for repository is so complex to the point it's hard to make changes.
// Since the usecase depends on an interface, you decided to use decorator
// to wrap the repository to include caching logic.
//
// FAQ:
// 1. Hey is this just decorator?
// - Proxy and decorators are very similar but there are 2 key differences:
//   1. You can create proxy object without the original object present.
//   2. Proxy handles access control, this mean it might change the behavior
//      of the contract. Decorator only adds new behavior.

use std::collections::HashMap;

use super::decorator::{User, UserRepository};

pub struct Cache {
    memory_cache: HashMap<String, String>,
}
impl Cache {
    pub fn new() -> Cache {
        Cache { memory_cache: HashMap::new() }
    }

    pub fn get(&self, name: String) -> Option<String> {
        let user_str = self.memory_cache.get(&name);
        if user_str.is_none() {
            return None;
        }

        let str = user_str.unwrap().clone();

        Some(str)
    }

    pub fn set(&mut self, key: String, value: String) {
        self.memory_cache.insert(key, value);
    }
}

#[allow(dead_code)]
pub struct UserRepositoryWithCache {
    pub cache: Cache,
    pub repo: Box<dyn UserRepository>,
}
impl UserRepository for UserRepositoryWithCache {
    fn find_user(&mut self, name: String) -> Option<User> {
        let key = format!("user:{}", name);
        let from_cache = self.cache.get(key.to_string());

        // return from cache
        if from_cache.is_some() {
            let cached_user: User = serde_json::from_str(from_cache.unwrap().as_str()).unwrap();

            return Some(cached_user);
        }

        let from_db = self.repo.find_user(name);
        self.cache.set(key.to_string(), serde_json::to_string(&from_db.as_ref()).unwrap());

        from_db
    }
}
