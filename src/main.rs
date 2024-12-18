use std::sync::{Arc, Barrier, Mutex};
use std::thread;
// use crate::{analyzer::Analyzer, fault::generate_faults, redundant::Redundant, sorting::bubble_sort};
use my_library::analyzer::Analyzer;
use my_library::fault::{generate_faults, fault_injector};
use my_library::sorting::{bubble_sort, simple_bubble_sort, non_redundant_bubble_sort};
use my_library::redundant::Redundant;
use std::fs::File;
use std::io::Write;
use std::time::Instant;
use rand::{thread_rng, Rng};
use std::mem::size_of;



// Funzione per generare un array casuale di variabili ridondanti
fn generate_random_array(n: usize) -> Vec<Redundant<i32>> {
    let mut rng = thread_rng(); // Generatore di numeri casuali
    (0..n).map(|_| Redundant::new(rng.gen_range(0..10))).collect() // Crea un array di variabili ridondanti con valori casuali
}




pub fn measure_memory_overhead() -> String {
    let redundant_var_size = size_of::<Redundant<i32>>();
    let non_redundant_var_size = size_of::<i32>();

    let num_elements = 50;
    // let non_redundant_array: Vec<i32> = redundant_array.iter().map(|x| x.get()).collect();

    let total_memory_redundant = redundant_var_size * num_elements;
    let total_memory_non_redundant = non_redundant_var_size * num_elements;

    let report = format!(
        "Size of Redundant<i32>: {} bytes\nSize of i32: {} bytes\nMemory overhead per element: {} bytes\nTotal memory with redundancy: {} bytes\nTotal memory without redundancy: {} bytes\nTotal memory overhead: {} bytes\n",
        redundant_var_size, non_redundant_var_size, redundant_var_size - non_redundant_var_size,
        total_memory_redundant, total_memory_non_redundant, total_memory_redundant - total_memory_non_redundant
    );

    let mut file = File::create("memory_overhead_report.txt").expect("Unable to create file");
    file.write_all(report.as_bytes()).expect("Unable to write data");

    report
}


pub fn measure_cpu_time_overhead() -> String {
    let num_elements = 50;
    let mut redundant_array = generate_random_array(num_elements);
    let mut non_redundant_array: Vec<i32> = redundant_array.iter().map(|x| x.get()).collect();

    let start_redundant = Instant::now();
    simple_bubble_sort(&mut redundant_array);
    let duration_redundant = start_redundant.elapsed();

    let start_non_redundant = Instant::now();
    non_redundant_bubble_sort(&mut non_redundant_array);
    let duration_non_redundant = start_non_redundant.elapsed();

    let report = format!(
        "CPU time with redundancy: {:?}\nCPU time without redundancy: {:?}\nCPU time overhead: {:?}\n",
        duration_redundant, duration_non_redundant, duration_redundant - duration_non_redundant
    );

    let mut file = File::create("cpu_time_overhead_report.txt").expect("Unable to create file");
    file.write_all(report.as_bytes()).expect("Unable to write data");

    report
}

fn main() {
    let num_elements = 50; // Numero di elementi nell'array
    let faults = generate_faults(num_elements, 1000); // Genera un elenco di fault
    let analyzer = Arc::new(Mutex::new(Analyzer::new())); // Crea un analizzatore condiviso

    // Measure memory overhead
    let memory_overhead_report = measure_memory_overhead();

    // Measure CPU time overhead
    let cpu_time_overhead_report = measure_cpu_time_overhead();

    // Save analyzer report to file
    analyzer.lock().unwrap().report_to_file("analyzer_report.txt");

    // Print reports to console (optional)
    println!("{}", memory_overhead_report);
    println!("{}", cpu_time_overhead_report);

    for fault in faults {
        // Crea un nuovo array per ogni fault
        let shared_array = Arc::new(Mutex::new(generate_random_array(num_elements))); // Array condiviso
        let start_barrier = Arc::new(Barrier::new(2)); // Barriera per sincronizzazione tra thread

        // Thread per iniettare fault
        let injector_array = Arc::clone(&shared_array);
        let injector_barrier = Arc::clone(&start_barrier);
        let injector_thread = thread::spawn(move || {
            fault_injector(injector_array, fault, injector_barrier); // Esegui l'iniezione del fault
        });

        // Bubble sort
        let sort_analyzer = Arc::clone(&analyzer);
        let bubble_sort_array = Arc::clone(&shared_array);
        
        // Aspetta che il fault injector inizi
        start_barrier.wait();
        
        // Esegui il bubble sort sull'array condiviso
        let sorting_success = bubble_sort(&mut bubble_sort_array.lock().unwrap(), &sort_analyzer);

        // Verifica risultato finale
        let correct_sort = bubble_sort_array
            .lock()
            .unwrap()
            .iter()
            .map(|x| x.get())
            .collect::<Vec<_>>()
            .windows(2)
            .all(|w| w[0] <= w[1]); // Controlla se l'array è ordinato

        // Logica per gestire i risultati
        let mut analyzer_lock = analyzer.lock().unwrap();
        
        // Aspetta il completamento dell'iniettore
        injector_thread.join().unwrap();

        // Registra i risultati in base al successo del sorting e alla correttezza
        if sorting_success {
            if correct_sort {
                analyzer_lock.log_result(true); // Esecuzione corretta
            } else {
                analyzer_lock.log_result(false); // Esecuzione errata
            }
        }
    }

    // Risultati finali
    analyzer.lock().unwrap().report(); // Stampa il report finale
    analyzer.lock().unwrap().report_to_file("analyzer_report.txt");
}