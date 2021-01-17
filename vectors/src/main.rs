#[derive(Debug)]
struct Person {
    name: String,
    age: Age,
}
#[derive(Debug)]
enum Age {
    Number(u8),
    Text(String),
}

fn main() {
    let people = vec![
        Person {
            name: String::from("Fede"),
            age: Age::Text(String::from("17 years old")),
        },
        Person {
            name: String::from("Agustin"),
            age: Age::Number(16),
        },
    ];

    for (i, person) in people.iter().enumerate() {
        println!("[{}] -> {}", i, person.name);
    }

    println!("{:#?}", people);
}
