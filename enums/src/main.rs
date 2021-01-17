fn main() {
    let num = 10;

    numbers_match(num);
}

fn numbers_match(num: i32) {
    match num {
        1 => println!("the number is one"),
        2 => println!("the number is two"),
        _ => println!("the number is {}", num),
    }
}
