use std::collections::HashMap;

#[derive(Debug)]
struct User {
    name: String,
    age: u8,
}

impl User {
    fn new(name: &str, age: u8) -> Self {
        Self {
            name: name.to_string(),
            age,
        }
    }
}

fn main() {
    let mut users = HashMap::new();

    users.insert(1, User::new("Fede", 17));
    users.insert(2, User::new("Pola", 16));

    for (key, user) in &users {
        println!("[{}] -> {} have {} years old", key, user.name, user.age);
    }
}
