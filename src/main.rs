use std::sync::{Arc, Barrier, Mutex};
use std::thread;
use my_library::analyzer::{Analyzer, measure_memory_overhead, measure_cpu_time_overhead};
use my_library::fault::{generate_faults, fault_injector};
use my_library::sorting::bubble_sort;

use my_library::utility::{load_config, generate_random_array};
use simplelog::*;
use std::fs::File;


fn main() {
    let config = load_config("config.toml");

    CombinedLogger::init(vec![
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create("fault_injection.log").unwrap(),
        ),
    ])
    .unwrap();

    let num_elements = config.num_elements;
    let fault_injection_time = config.fault_injection_time;
    let max_bit_to_flip = config.max_bit_to_flip;
    let faults = generate_faults(num_elements, config.num_faults, max_bit_to_flip,fault_injection_time);
    let analyzer = Arc::new(Mutex::new(Analyzer::new())); // Crea un analizzatore condiviso

    // Measure memory overhead
    let memory_overhead_report = measure_memory_overhead(num_elements);

    // Measure CPU time overhead
    let cpu_time_overhead_report = measure_cpu_time_overhead(num_elements*100);

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
        // let sort_analyzer = Arc::clone(&analyzer);
        let bubble_sort_array = Arc::clone(&shared_array);
        
        // Aspetta che il fault injector inizi
        start_barrier.wait();
        
        // Esegui il bubble sort sull'array condiviso
        let result = bubble_sort(&mut bubble_sort_array.lock().unwrap());



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

        if result.is_ok() {
            if correct_sort {
                analyzer_lock.log_result(true); // Esecuzione corretta
            } else {
                analyzer_lock.log_result(false); // Esecuzione errata
            }
        } else {
            log::info!("Analyzer: {}", result.unwrap_err());
            analyzer_lock.log_fault(); // Fault durante il sorting
        }
    }

    // Risultati finali
    analyzer.lock().unwrap().report(); // Stampa il report finale
    analyzer.lock().unwrap().report_to_file("analyzer_report.txt");
}