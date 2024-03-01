// Decorator is a pattern that allows you to attach new behavior
// to an object using a wrapper object without modifying the original object
//
// In this example, we have a data repository that fetches data from db. Currently,
// the app is running well on production. After a few months, you realized
// that the app is a bit slow, so you want to add caching. However, the code 
// for repository is so complex to the point it's hard to make changes.
// Since the usecase depends on an interface, you decided to use decorator
// to wrap the repository to include caching logic

pub struct User {
    pub name: String,
    pub age: u16,
}
pub trait UserRepository {
    fn find_user(&self, name: String) -> Option<User>;
}

pub struct UserPostgreRepository {}
impl UserRepository for UserPostgreRepository {
    fn find_user(&self, _: String) -> Option<User> {
        // imagine if we query the db
        Some(User { name: "John Doe".to_string(), age: 37 })
    }
}

pub struct Cache {}
impl Cache {
    pub fn get(&self, _: String) -> Option<String> {
        Some("{ \"name\": \"John Doe\", \"age\": 38 }".to_string())
    }

    pub fn set(&self, _: String, _: String) {
        // set to cache
    }
}

#[allow(dead_code)]
pub struct UserRepositoryWithCache {
    pub cache: Cache,
    pub repo: Box<dyn UserRepository>,
}

impl UserRepository for UserRepositoryWithCache {
    fn find_user(&self, name: String) -> Option<User> {
        let key = format!("user:{}", name);
        let from_cache = self.cache.get(key.to_string());

        // return from cache
        if from_cache.is_some() {
            return Some(User { name: "John Doe".to_string(), age: 38 });
        }

        let from_db = User { name: "John Doe".to_string(), age: 37 };
        self.cache.set(key.to_string(), "{ \"name\": \"John Doe\", \"age\": 37 }".to_string());

        return Some(from_db);
    }
}
