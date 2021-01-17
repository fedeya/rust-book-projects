#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 10,
        height: 5,
    };

    let rect2 = Rectangle {
        width: 20,
        height: 10,
    };

    let square = Rectangle::square(20);

    println!("{:#?}", rect);
    println!("{}", rect.area());
    println!("{}", rect2.can_hold(&rect));
    println!("{:#?}", square);
}
