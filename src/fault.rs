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

pub fn generate_faults(array_len: usize, num_faults: usize) -> Vec<Fault> {
    let mut rng = rand::thread_rng();
    (0..num_faults)
        .map(|_| Fault {
            index: rng.gen_range(0..array_len),
            bit_to_flip: rng.gen_range(0..10),
            time: rng.gen_range(0..50),
        })
        .collect()
}

// Funzione per iniettare un fault in una variabile ridondante
fn inject_fault (var: &mut Redundant<i32>, bit_position: u8) {
    var.duplicate ^= 1 << bit_position; // Inverte il bit specificato nella variabile duplicata
    println!("Injected fault: Flipping bit {} of duplicate (current value: {}, previous: {}).", bit_position, var.duplicate, var.value);
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
    print!("Fault injection: ");
    if fault.index < arr.len() {
        inject_fault(&mut arr[fault.index], fault.bit_to_flip); // Inietta il fault
        println!(
            "Fault injected: variable at index {} affected, bit {} flipped.",
            fault.index, fault.bit_to_flip
        );
    }
}