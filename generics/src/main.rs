struct Rectangle<T> {
    width: T,
    height: T,
}

impl Rectangle<i32> {
    fn area(&self) -> i32 {
        return self.width * self.height;
    }
}

fn main() {
    let rect = Rectangle {
        width: 20,
        height: 5,
    };

    rect.area();

    println!("Hello, world!");
}
