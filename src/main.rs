use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct RedundantVar<T> {
    first: T,
    second: T,
}

impl<T: PartialEq + Copy + std::fmt::Debug> RedundantVar<T> {
    fn new(value: T) -> Self {
        RedundantVar {
            first: value,
            second: value,
        }
    }

    fn set(&mut self, value: T) {
        self.first = value;
        self.second = value;
    }

    fn get(&self) -> T {
        if self.is_valid() {
            self.first
        } else {
            panic!("Fault detected: values are not congruent!");
        }
    }

    fn is_valid(&self) -> bool {
        self.first == self.second
    }
}

fn main() {

    let mut a = RedundantVar::new(3);

    a.set(5);
   
    // Modifica direttamente il campo `first`
    a.first = 7;

    a.get();  // causa un panic correttamente



}

