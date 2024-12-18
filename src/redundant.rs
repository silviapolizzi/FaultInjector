#[derive(Clone, Debug)]
pub struct Redundant<T: Copy + PartialEq> {
    pub value: T,
    pub duplicate: T,
}

impl<T: Copy + PartialEq> Redundant<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            duplicate: value,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.value == self.duplicate
    }

    pub fn get(&self) -> T {
        self.value
    }
}
