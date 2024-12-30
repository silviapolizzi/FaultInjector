
use crate::redundant::Redundant;


pub fn bubble_sort(arr: &mut [Redundant<i32>]) -> Result<bool, String> {
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