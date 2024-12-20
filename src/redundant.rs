#[derive(Clone, Debug)]
pub struct Redundant<T: Copy + PartialEq> {
    pub value: T,
    pub duplicate: T,
}

impl<T: Copy + PartialEq + std::fmt::Debug> Redundant<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            duplicate: value,
        }
    }

    fn is_valid(&self) -> bool {
        self.value == self.duplicate
    }

    pub fn get(&self) -> Result<T, String> {
        if self.is_valid() {
            Ok(self.value)
        } else {
            println!("Variable is invalid: value = {:?}, duplicate = {:?}", self.value, self.duplicate);
            Err(format!(
                "Variable is invalid: value = {:?}, duplicate = {:?}",
                self.value, self.duplicate
            ))
        }
    }


    pub fn set(&mut self, new_value: T) {
        self.value = new_value;
        self.duplicate = new_value;
    }
}
