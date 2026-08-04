struct Countdown {
    count: u32,
}

impl Countdown {
    fn new(count: u32) -> Self {
        Countdown { count }
    }
}

impl Iterator for Countdown {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count > 0 {
            let current = self.count;
            self.count -= 1;
            Some(current) // Return the next value
        } else {
            None // Signal that iteration is complete
        }
    }
}

fn main() {
    let timer = Countdown::new(5);

    // Iterators work natively with Rust's for loops
    for num in timer {
        println!("{}", num); // Prints 3, then 2, then 1
    }

    let sum: u32 = Countdown::new(5)
        .filter(|x| x % 2 == 0) // Adapter: Lazy modification
        .map(|x| x * 10) // Adapter: Lazy modification
        .sum(); // Consumer: Executes code and yields result

    println!("{}", sum)
}
