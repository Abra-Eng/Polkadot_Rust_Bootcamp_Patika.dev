fn main() {
    let mut fibonacci = Fibonacci {
        current: 0, 
        next:1
    };

    for _ in 0..5 {
        println!("{}", fibonacci.next().unwrap());
    }


    let countdown = Countdown {remaining: 32};

    for i in countdown {
        println!("remaining: {}", i);
    }
}

// an iterator is an object that implements Iterotor trait, they only computes the next value

// trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;  primary method
// } 

struct Fibonacci {
    current: u32,
    next: u32,
}

impl Iterator for Fibonacci { 
    type Item = u32; // Associated item

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        self.current = self.next;
        self.next = self.current + self.next;

        Some(current)
    }
}

// Creating Custom Iterators

struct Countdown {
    remaining: i32,
}

impl Iterator for Countdown {
    type Item = i32; // Associated item

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining > 0 {
            let current = self.remaining;
            self.remaining -= 1;
            Some(current)
        } else {
            None
        }

    }
}
