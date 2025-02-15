
use crate::redundant::Redundant;


pub fn redundant_bubble_sort(arr: &mut [Redundant<i32>]) -> Result<bool, String> {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            let current = arr[j].get()?;
            let next = arr[j + 1].get()?;
            if current > next {
                let temp = current;
                arr[j].set(next);
                arr[j + 1].set(temp);
            }
        }
    }
    Ok(true)
}

use std::sync::{Arc, Mutex};

pub fn bubble_sort(shared_arr: &Arc<Mutex<Vec<Redundant<i32>>>>) -> Result<bool, String> {
    // Determina la dimensione dell'array
    let n = {
        let arr = shared_arr.lock().unwrap();
        arr.len()
    };

    for i in 0..n {
        for j in 0..n - i - 1 {
            // Acquisisci il lock per leggere gli elementi j e j+1
            let (current, next) = {
                let arr = shared_arr.lock().unwrap();
                (arr[j].get()?, arr[j + 1].get()?)
            };

            if current > next {
                // Per effettuare lo swap, acquisisci nuovamente il lock
                let mut arr = shared_arr.lock().unwrap();
                // Copia temporaneamente i valori, evitando borrows prolungati
                let temp = arr[j].get()?;
                let next_val = arr[j + 1].get()?;
                arr[j].set(next_val);
                arr[j + 1].set(temp);
            }
        }
    }
    Ok(true)
}



pub fn non_redundant_bubble_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
            
                // Scambio esplicito con variabile temporanea
                let temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
            
            }
        }
    }
}