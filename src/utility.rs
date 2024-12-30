use serde::Deserialize;
use std::fs;
use rand::{thread_rng, Rng};
use crate::redundant::Redundant;


#[derive(Deserialize)]
pub struct Config {
    pub num_elements: usize,
    pub num_faults: usize,
    pub max_bit_to_flip: u8,
    pub fault_injection_time: usize, // Tempo di iniezione in millisecondi o nanosecondi
}
pub fn load_config(filename: &str) -> Config {
    let config_data = fs::read_to_string(filename).expect("Unable to read config file");
    toml::from_str(&config_data).expect("Invalid config format")
}




// Funzione per generare un array casuale di variabili ridondanti
pub fn generate_random_array(n: usize) -> Vec<Redundant<i32>> {
    let mut rng = thread_rng(); // Generatore di numeri casuali
    (0..n).map(|_| Redundant::new(rng.gen_range(0..10))).collect() // Crea un array di variabili ridondanti con valori casuali
}


