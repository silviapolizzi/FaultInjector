use std::sync::{Arc, Barrier, Mutex};
use std::thread;
use my_library::analyzer::{Analyzer, measure_memory_overhead, measure_cpu_time_overhead};
use my_library::fault::{generate_faults, fault_injector};
use my_library::sorting::bubble_sort;

use my_library::utility::{load_config, generate_random_array};
use simplelog::*;
use std::fs::{File, create_dir_all};


fn main() {
    let config = load_config("config.toml");
    create_dir_all("results").expect("Unable to create results directory");
    CombinedLogger::init(vec![
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create("results/fault_injection.log").unwrap(),
        ),
    ])
    .unwrap();

    let num_elements = config.num_elements;
    let fault_injection_time = config.fault_injection_time;
    let max_bit_to_flip = config.max_bit_to_flip;
    let faults = generate_faults(num_elements, config.num_faults, max_bit_to_flip,fault_injection_time);
    let mut analyzer= Analyzer::new(); // Crea un nuovo Analyzer

    // Misura l'overhead di memoria
    let memory_overhead_report = measure_memory_overhead(num_elements);

    // Misura l'overhead di CPU time
    let cpu_time_overhead_report = measure_cpu_time_overhead(num_elements);

    // Salva il report dell'analizzatore su file
    analyzer.report_to_file("analyzer_report.txt");

    // Stampa i report sulla console 
    println!("{}", memory_overhead_report);
    println!("{}", cpu_time_overhead_report);

    for fault in faults {
        // Crea un nuovo array per ogni fault
        let shared_array = Arc::new(Mutex::new(generate_random_array(num_elements))); // Array condiviso
        let start_barrier = Arc::new(Barrier::new(2)); // Barriera per sincronizzazione tra (2) thread

        // Thread per iniettare fault
        let injector_array = Arc::clone(&shared_array);
        let injector_barrier = Arc::clone(&start_barrier);
        let injector_thread = thread::spawn(move || {
            fault_injector(injector_array, fault, injector_barrier); // Esegue l'iniezione del fault
        });

        // Bubble sort 

        let bubble_sort_array = Arc::clone(&shared_array);
        start_barrier.wait();  // Aspetta che il fault injector inizi
        let result = bubble_sort(&bubble_sort_array); // Esegue il bubble sort sull'array condiviso
        // result è un Result<bool, String>, contiene l'informazione che l'ordinamento sia andato a buon fine senza errori o ci sono stati fault
        // bubble_sort_array a questo punto è stato ordinato


        // Aspetta il completamento dell'iniettore
        injector_thread.join().unwrap();

        // Verifica risultato finale
        let correct_sort = bubble_sort_array
            .lock()
            .unwrap()
            .iter()
            .map(|x| x.get())
            .collect::<Vec<_>>()
            .windows(2)
            .all(|w| w[0] <= w[1]); // Controlla se l'array è ordinato


        if result.is_ok() {
            if correct_sort {
                analyzer.log_result(true); // Esecuzione corretta
            } else {
                analyzer.log_result(false); // Esecuzione errata
            }
        } else {
            log::info!("Analyzer: {}", result.unwrap_err());
            analyzer.log_fault(); // Fault durante il sorting
        }
    }

    // Risultati finali
    analyzer.report(); // Stampa il report finale
    analyzer.report_to_file("analyzer_report.txt");
}