use std::fs::File;
use std::io::Write;

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

