use std::sync::{Arc, Mutex};
use crate::analyzer::Analyzer;
use crate::redundant::Redundant;



// Funzione per eseguire il bubble sort su un array di variabili ridondanti
pub fn bubble_sort(arr: &mut [Redundant<i32>], analyzer: &Arc<Mutex<Analyzer>>) -> bool {
    print!("Sorting array: ");
    let n = arr.len(); // Ottieni la lunghezza dell'array
    for i in 0..n {
        for j in 0..n - i - 1 {
            // Controlla se ci sono fault nelle variabili
            if !arr[j].is_valid() || !arr[j + 1].is_valid() {
                println!("Fault detected during sorting at index {} and {}!", j, j + 1);
                analyzer.lock().unwrap().log_fault(); // Registra il fault
                return false; // Il sorting fallisce a causa di un fault
            }
            // Esegui lo scambio se l'elemento corrente è maggiore del successivo
            if arr[j].get() > arr[j + 1].get() {
                arr.swap(j, j + 1);
            }
        }
    }
    println!("sorting done!");
    true // Sorting completato con successo
}

pub fn simple_bubble_sort(arr: &mut [Redundant<i32>]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j].get() > arr[j + 1].get() {
                arr.swap(j, j + 1);
            }
        }
    }
}


pub fn non_redundant_bubble_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}