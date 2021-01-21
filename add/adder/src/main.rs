use add_one::add_one;
use add_two::add_two;

fn main() {
    println!("Hello, world!");
    let x = add_one(1);
    let y = add_two(2);

    println!("{} {}", x, y);
}
