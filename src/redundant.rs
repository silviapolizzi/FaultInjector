pub struct Redundant<T> {
    pub first: T,
    pub second: T,
}

impl<T: Copy + PartialEq + std::fmt::Debug> Redundant<T> {
    pub fn new(first: T) -> Self {
        Self {
            first,
            second: first, 
        }
    }
    fn is_valid(&self) -> bool {
        self.first == self.second  // PartialEq needed here
    }

    pub fn get(&self) -> Result<T, String> {
        if self.is_valid() {
            Ok(self.first)
        } else {
            Err(format!( "Variable is invalid: first = {:?}, second = {:?}", self.first, self.second)) // Debug needed here
        }
    }


    pub fn set(&mut self, new_value: T) {
        self.first = new_value;
        self.second = new_value;
    }
}
