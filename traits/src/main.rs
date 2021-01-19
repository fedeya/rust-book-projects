trait Alive {
    fn say_hello(&self);
    fn data(&self);
}

#[derive(Debug)]
enum Gender {
    Male,
    Female,
}

struct Dog {
    name: String,
    gender: Gender,
}

struct Human {
    name: String,
    gender: Gender,
}

impl Alive for Dog {
    fn say_hello(&self) {
        println!("guau guau");
    }

    fn data(&self) {
        println!("guagua {} guaguagua {:?}", self.name, self.gender);
    }
}

impl Alive for Human {
    fn say_hello(&self) {
        println!("hello!")
    }

    fn data(&self) {
        println!("hello i'm {} and i'm {:?}", self.name, self.gender);
    }
}

fn main() {
    let fede = Human {
        gender: Gender::Male,
        name: "Fede".to_string(),
    };

    let chiqui = Dog {
        gender: Gender::Female,
        name: "Chiqui".to_string(),
    };

    hello_and_data(fede);
    hello_and_data(chiqui);
}

fn hello_and_data<T: Alive>(alive: T) {
    alive.say_hello();
    alive.data();
}
