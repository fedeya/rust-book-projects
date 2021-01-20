struct Counter {
    count: u32,
}

impl Counter {
    pub fn new() -> Self {
        Self { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

pub fn run() {
    let counter = Counter::new();

    // println!("{:?}", counter.next());
    // println!("{:?}", counter.next());
    // println!("{:?}", counter.next());
    // println!("{:?}", counter.next());
    // println!("{:?}", counter.next());
    // println!("{:?}", counter.next());

    let sum: u32 = counter.sum();

    println!("all value sum {}", sum);
}
