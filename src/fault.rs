use std::sync::{Arc, Barrier, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;

use crate::redundant::Redundant;


pub struct Fault {
    pub index: usize,
    pub bit_to_flip: u8,
    pub time: usize,
}


pub fn generate_faults(
    array_len: usize, num_faults: usize, max_bit_to_flip: u8, max_time: usize,) -> Vec<Fault> {
    let mut rng = rand::thread_rng();
    (0..num_faults)
        .map(|_| Fault {
            index: rng.gen_range(0..array_len), // Indice casuale nell'array
            bit_to_flip: rng.gen_range(0..max_bit_to_flip), // Bit casuale da flippare
            time: rng.gen_range(0..max_time), // Tempo casuale
        })
        .collect()
}


// Funzione per iniettare un fault in una variabile ridondante
fn inject_fault (var: &mut Redundant<i32>, bit_position: u8) {
    var.first ^= 1 << bit_position; // Inverte il bit specificato nella variabile duplicata
    log::info!("Injected fault: Flipping bit {} of second value(current value: {}, previous: {}).", bit_position, var.first, var.second);
}


// Funzione per iniettare fault nell'array
pub fn fault_injector(
    arr: Arc<Mutex<Vec<Redundant<i32>>>>, // Array condiviso
    fault: Fault, // Fault da iniettare
    start_barrier: Arc<Barrier>, // Barriera per sincronizzazione
) {
    start_barrier.wait(); // Aspetta che tutti i thread siano pronti
    thread::sleep(Duration::from_millis(fault.time as u64)); // Simula un ritardo
    let mut arr = arr.lock().unwrap(); // Ottieni il lock sull'array
    log::info!("Fault injection: ");
    if fault.index < arr.len() {
        inject_fault(&mut arr[fault.index], fault.bit_to_flip); // Inietta il fault
        log::info!(
            "variable at index {} affected, bit {} flipped.",
            fault.index, fault.bit_to_flip
        );
    }
}