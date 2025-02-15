use std::fs::File;
use std::io::Write;
use cpu_time::ThreadTime;
use std::mem::size_of;
use crate::redundant::Redundant;
use crate::sorting::{redundant_bubble_sort, non_redundant_bubble_sort};
use crate::utility::generate_random_array;


pub struct Analyzer {
    fault_count: usize,
    correct_runs: usize,
    incorrect_runs: usize,
}

impl Analyzer {
    pub fn new() -> Self {
        Self {
            fault_count: 0,
            correct_runs: 0,
            incorrect_runs: 0,
        }
    }

    pub fn log_fault(&mut self) {
        self.fault_count += 1;
        log::info!("Fault detected!");
    }

    pub fn log_result(&mut self, correct: bool) {
        if correct {
            self.correct_runs += 1;
        } else {
            self.incorrect_runs += 1;
        }
    }
    
    // Metodo per stampare il report finale
    pub fn report(&self) {
        println!("--- Analyzer Report ---");
        println!("Faults detected: {}", self.fault_count);
        println!("Correct runs: {}", self.correct_runs);
        println!("Incorrect runs: {}", self.incorrect_runs);

    }

    pub fn report_to_file(&self, file_path: &str) {
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


pub fn measure_memory_overhead(num_elements: usize) -> String {
    let redundant_var_size = size_of::<Redundant<i32>>();
    let non_redundant_var_size = size_of::<i32>();

    let total_memory_redundant = redundant_var_size * num_elements;
    let total_memory_non_redundant = non_redundant_var_size * num_elements;

    let report = format!(
        "Number of elements: {}\nSize of Redundant<i32>: {} bytes\nSize of i32: {} bytes\nMemory overhead per element: {} bytes\nTotal memory with redundancy: {} bytes\nTotal memory without redundancy: {} bytes\nTotal memory overhead: {} bytes\n",num_elements,
        redundant_var_size, non_redundant_var_size, redundant_var_size - non_redundant_var_size,
        total_memory_redundant, total_memory_non_redundant, total_memory_redundant - total_memory_non_redundant
    );

    let mut file = File::create("memory_overhead_report.txt").expect("Unable to create file");
    file.write_all(report.as_bytes()).expect("Unable to write data");

    report
}

pub fn measure_cpu_time_overhead(num_elements: usize) -> String {
    let mut redundant_array = generate_random_array(num_elements);
    let mut non_redundant_array: Vec<i32> = redundant_array.iter().map(|x| x.get().unwrap()).collect();

    // Misura CPU time per il sorting con ridondanza
    let thread_time_start_redundant = ThreadTime::now();
    let _ = redundant_bubble_sort(&mut redundant_array);
    let duration_redundant = thread_time_start_redundant.elapsed();

    // Misura CPU time per il sorting senza ridondanza
    let thread_time_start_non_redundant = ThreadTime::now();
    non_redundant_bubble_sort(&mut non_redundant_array);
    let duration_non_redundant = thread_time_start_non_redundant.elapsed();

    // Calcola l'overhead
    let overhead = duration_redundant - duration_non_redundant;

    let report = format!(
        "Number of elements: {}\nCPU time with redundancy: {:?}\nCPU time without redundancy: {:?}\nCPU time overhead: {:?}\n",num_elements,
        duration_redundant, duration_non_redundant, overhead
    );

    let mut file = File::create("cpu_time_overhead_report.txt").expect("Unable to create file");
    file.write_all(report.as_bytes()).expect("Unable to write data");

    report
}

