fn main() {
    let name = String::from("Fede");
    let number: u32 = "8".parse().expect("error parsing the number");

    println!("{} {}", name, number);
}
