struct Countdown {
    low: i32,
    high: i32,
}

// 1. Must implement standard Iterator first
impl Iterator for Countdown {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.low <= self.high {
            let res = self.low;
            self.low += 1;
            Some(res)
        } else {
            None
        }
    }
}

// 2. Implement DoubleEndedIterator by providing next_back
impl DoubleEndedIterator for Countdown {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.low <= self.high {
            let res = self.high;
            self.high -= 1;
            Some(res)
        } else {
            None
        }
    }
}

fn main() {
    let mut counter = Countdown { low: 1, high: 5 };

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next_back(), Some(5));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next_back(), Some(4));
    assert_eq!(counter.next(), Some(3)); // low and high have crossed
}
