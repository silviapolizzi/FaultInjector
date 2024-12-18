use rand::prelude::*;
use std::fmt::Debug;
use std::sync::{Arc, Mutex, Barrier};
use std::thread;
use std::time::Duration;
use std::time::Instant;
use std::mem::size_of;
use std::fs::File;
use std::io::Write;

// Struttura per gestire l'analizzatore
struct Analyzer {
    fault_count: usize,      // Conteggio dei fault rilevati
    correct_runs: usize,     // Conteggio delle esecuzioni corrette
    incorrect_runs: usize,   // Conteggio delle esecuzioni errate
}

impl Analyzer {
    // Costruttore per inizializzare un nuovo Analyzer
    fn new() -> Self {
        Self {
            fault_count: 0,
            correct_runs: 0,
            incorrect_runs: 0,
        }
    }

    // Metodo per registrare un fault
    fn log_fault(&mut self) {
        self.fault_count += 1;
    }

    // Metodo per registrare il risultato di un'esecuzione
    fn log_result(&mut self, correct: bool) {
        if correct {
            self.correct_runs += 1; // Incrementa se l'esecuzione è corretta
        } else {
            self.incorrect_runs += 1; // Incrementa se l'esecuzione è errata
        }
    }

    // Metodo per stampare il report finale
    fn report(&self) {
        println!("--- Analyzer Report ---");
        println!("Faults detected: {}", self.fault_count);
        println!("Correct runs: {}", self.correct_runs);
        println!("Incorrect runs: {}", self.incorrect_runs);

    }

    fn report_to_file(&self, file_path: &str) {
        let report = self.generate_report();
        let mut file = File::create(file_path).expect("Unable to create file");
        file.write_all(report.as_bytes()).expect("Unable to write data");
    }

    fn generate_report(&self) -> String {

        // Format the report
        format!(
            "Analyzer Report\n\nFault Count: {}\nTotal Checks: {}\nValid Checks: {}\nInvalid Checks: {}\n",
            self.fault_count, self.correct_runs + self.incorrect_runs + self.fault_count, self.correct_runs, self.incorrect_runs
        )
    }
}

// Funzione per eseguire il bubble sort su un array di variabili ridondanti
fn bubble_sort(arr: &mut [Redundant<i32>], analyzer: &Arc<Mutex<Analyzer>>) -> bool {
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

// Funzione per iniettare fault nell'array
fn fault_injector(
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

// Struttura per rappresentare un fault
struct Fault {
    index: usize,     // Indice dell'elemento da modificare
    bit_to_flip: u8,  // Bit da invertire
    time: usize,      // Tempo di attesa prima di iniettare il fault
}

// Funzione per generare un elenco di fault casuali
fn generate_faults(array_len: usize, num_faults: usize) -> Vec<Fault> {
    let mut rng = thread_rng(); // Generatore di numeri casuali
    (0..num_faults)
        .map(|_| Fault {
            index: rng.gen_range(0..array_len), // Indice casuale
            bit_to_flip: rng.gen_range(0..10),   // Bit casuale da invertire
            time: rng.gen_range(0..50),           // Tempo casuale di attesa
        })
        .collect() // Raccoglie i fault in un vettore
}

// Funzione per iniettare un fault in una variabile ridondante
fn inject_fault (var: &mut Redundant<i32>, bit_position: u8) {
    var.duplicate ^= 1 << bit_position; // Inverte il bit specificato nella variabile duplicata
    println!("Injected fault: Flipping bit {} of duplicate (current value: {}, previous: {}).", bit_position, var.duplicate, var.value);
}

// Struttura per variabile ridondante
#[derive(Clone, Debug)]
struct Redundant<T: Copy + PartialEq> {
    value: T,     // Valore originale
    duplicate: T, // Valore duplicato
}

impl<T: Copy + PartialEq + Debug> Redundant<T> {
    // Costruttore per creare una nuova variabile ridondante
    fn new(value: T) -> Self {
        Self {
            value,
            duplicate: value, // Inizializza il duplicato con il valore originale
        }
    }

    // Controlla se la variabile è valida (valore originale uguale al duplicato)
    fn is_valid(&self) -> bool {
        self.value == self.duplicate
    }

    // Restituisce il valore originale
    fn get(&self) -> T {
        self.value
    }
}

// Funzione per generare un array casuale di variabili ridondanti
fn generate_random_array(n: usize) -> Vec<Redundant<i32>> {
    let mut rng = thread_rng(); // Generatore di numeri casuali
    (0..n).map(|_| Redundant::new(rng.gen_range(0..10))).collect() // Crea un array di variabili ridondanti con valori casuali
}


fn measure_memory_overhead() -> String {
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

fn simple_bubble_sort(arr: &mut [Redundant<i32>]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j].get() > arr[j + 1].get() {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn measure_cpu_time_overhead() -> String {
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

fn non_redundant_bubble_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
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