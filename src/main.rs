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

    fn get(&self) -> Result<T, String> {
        if self.first != self.second {
            Err(format!(
                "Incongruenza rilevata! first: {:?}, second: {:?}",
                self.first, self.second
            ))
        } else {
            Ok(self.first)
        }
    }
}

fn main() {

    let mut a = RedundantVar::new(3);

    // Gestisci l'output di get in modo più elegante
    match a.get() {
        Ok(value) => println!("Il valore è: {:?}", value),
        Err(err) => println!("Errore: {}", err),
    }
    a.set(5);

    match a.get() {
        Ok(value) => println!("Il valore è: {:?}", value),
        Err(err) => println!("Errore: {}", err),
    }

    // Modifica direttamente il campo `first`
    a.first = 7;
    // Gestisci l'output di get
    match a.get() {
        Ok(value) => println!("Il valore è: {:?}", value),
        Err(err) => println!("Errore: {}", err),

    }

}