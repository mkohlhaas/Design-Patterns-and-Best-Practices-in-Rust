// ============ //
// A. Countdown //
// ============ //

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

// =================== //
// B. History Iterator //
// =================== //

#[derive(Debug)]
pub struct Calculation; // Placeholder

pub struct HistoryIterator<'a> {
    // Wrap Rust's native slice iterator directly
    pub history: std::slice::Iter<'a, Calculation>,
}

impl<'a> Iterator for HistoryIterator<'a> {
    type Item = &'a Calculation;

    fn next(&mut self) -> Option<Self::Item> {
        // Delegate directly to the underlying iterator
        self.history.next()
    }
}

impl<'a> HistoryIterator<'a> {
    // Constructor that accepts a slice reference
    pub fn new(history: &'a [Calculation]) -> Self {
        Self {
            history: history.iter(),
        }
    }
}

// ===== //
// Usage //
// ===== //

fn main() {
    {
        let timer = Countdown::new(5);

        // Iterators work natively with Rust's for loops
        for num in timer {
            println!("{}", num); // Prints 3, then 2, then 1
        }

        let sum: u32 = Countdown::new(5)
            .filter(|x| x % 2 == 0) // Adapter: Lazy modification
            .map(|x| x * 10) // Adapter: Lazy modification
            .sum(); // Consumer: Executes code and yields result

        println!("{}", sum);
    }

    println!();

    {
        // 1. Create some sample data
        let calculation_storage = vec![Calculation, Calculation, Calculation];

        // 2. Initialize the HistoryIterator by passing a slice reference
        let history_iter = HistoryIterator::new(&calculation_storage);

        // 3. Use it like any other iterator
        for calc in history_iter {
            // Process your calculation here
            println!("{:?}", calc)
        }
    }
}
