#[derive(PartialEq, Debug)]
enum ShoeStyle {
    Sneaker,
    Sandal,
    Boot,
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: ShoeStyle,
}

pub fn run() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: ShoeStyle::Sneaker,
        },
        Shoe {
            size: 13,
            style: ShoeStyle::Sandal,
        },
        Shoe {
            size: 10,
            style: ShoeStyle::Boot,
        },
    ];

    let size = 10;

    let filtered_shoes: Vec<_> = shoes.iter().filter(|s| s.size == size).collect();

    println!("{:?}", filtered_shoes);
}
