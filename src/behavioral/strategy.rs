// Strategy is a design pattern that allows you to apply
// implementation details in separate class to make them interchangable.
//
// In this example, we have a circle that can be exported into multiple
// type of data, e.g JSON, YAML, TOML, etc. Since it will clutter the code
// if we implement all the logics in the circle struct, we can instead
// create a constructor that accepts Exporter trait.
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

pub trait Exporter {
    fn export(&self, data: String);
}

pub struct KeJSONGan;
impl Exporter for KeJSONGan {
    fn export(&self, _: String) {
        println!("Exporting data as JSON");
    }
}

pub struct KeYAMLGan;
impl Exporter for KeYAMLGan {
    fn export(&self, _: String) {
        println!("Exporting data as YAML");
    }
}

pub struct KeTOMLGan;
impl Exporter for KeTOMLGan {
    fn export(&self, _: String) {
        println!("Exporting data as TOML");
    }
}

pub fn export_data(data: String, strategy: impl Exporter) {
    strategy.export(data);
}
