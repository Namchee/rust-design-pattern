// Strategy is a design pattern that allows you to apply
// implementation details in separate class to make them interchangable.
//
// In a language that supports functions and closure, we just need to pass
// function to it.
//
// The easiest example of strategy is anonymous function, for example
// .find()

pub struct User {
    pub id: String,
    pub name: String,
}

pub fn is_user_exist(users: Vec<User>, id: String) -> bool {
    let u = users.iter().find(|user| user.id == id);
    if u.is_none() {
        return false;
    }

    true
}
