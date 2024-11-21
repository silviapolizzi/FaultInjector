

trait Redundant<T> {
    fn new(value: T) -> Self;
    fn set(&mut self, value: T);
    fn get(&self) -> T;
    fn is_valid(&self) -> bool;
}

trait FaultInjectable {
    fn inject_bit_fault(&mut self, bit_position: u8);
}

#[derive(Debug, Clone)]
struct RedundantVar<T: Clone + PartialEq + FaultInjectable> {
    original: T,
    duplicate: T,
}

impl<T: Clone + PartialEq + FaultInjectable> Redundant<T> for RedundantVar<T> {
    fn new(value: T) -> Self {
        RedundantVar {
            original: value.clone(),
            duplicate: value,
        }
    }

    fn set(&mut self, value: T) {
        self.original = value.clone();
        self.duplicate = value;
    }

    fn get(&self) -> T {
        if self.is_valid() {
            self.original.clone()
        } else {
            panic!("Fault detected: values are not congruent!");
        }
    }

    fn is_valid(&self) -> bool {
        self.original == self.duplicate
    }
}


impl FaultInjectable for i32 {
    fn inject_bit_fault(&mut self, bit_position: u8) {
        let mask = 1 << bit_position;
        *self ^= mask;
    }
}


fn bubble_sort(arr: &mut Vec<RedundantVar<i32>>) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..(n - i - 1) {
            if arr[j].get() > arr[j + 1].get() {
                let temp = arr[j].get();
                let temp1 = arr[j+1].get();
                arr[j].set(temp1);
                arr[j + 1].set(temp);
            }
        }
    }

}

use rand::{Rng, seq::SliceRandom};

#[derive(Debug)]
struct Fault {
    variable_name: String,
    bit_to_flip: u8,
    time: u64,
}

fn inject_fault(var: &mut RedundantVar<i32>, bit_position: u8) {
    let max_bit_position = 4; // Limita il cambiamento ai bit meno significativi

    if bit_position <= max_bit_position {
        var.duplicate ^= 1 << bit_position; // Inverti solo i primi bit
        println!(
            "Dopo l'iniezione: original = {}, duplicate = {}",
            var.original, var.duplicate
        );
    }
}




fn generate_faults(variables: Vec<String>, num_faults: usize) -> Vec<Fault> {
    let mut rng = rand::thread_rng();
    let mut faults = Vec::new();
    for _ in 0..num_faults {
        let var_name = variables.choose(&mut rng).unwrap().clone();
        let bit_to_flip = rng.gen_range(0..32); // Supponendo variabili a 32 bit
        let time = rng.gen_range(0..1000); // Tempo casuale
        faults.push(Fault {
            variable_name: var_name,
            bit_to_flip,
            time,
        });
    }
    faults
}

#[derive(Debug)]
struct Analyzer {
    total_runs: usize,
    faults_detected: usize,
    incorrect_results: usize,
    correct_results: usize,
}

impl Analyzer {
    fn new() -> Self {
        Analyzer {
            total_runs: 0,
            faults_detected: 0,
            incorrect_results: 0,
            correct_results: 0,
        }
    }

    fn analyze_run(&mut self, result: Option<Vec<i32>>, is_fault_detected: bool) {
        self.total_runs += 1;
        if is_fault_detected {
            self.faults_detected += 1;
        } else if let Some(res) = result {
            // Confrontiamo il risultato con l'output atteso
            let expected = vec![1, 2, 3, 4, 5]; // Cambiare con il risultato atteso
            if res == expected {
                self.correct_results += 1;
            } else {
                self.incorrect_results += 1;
            }
        }
    }

    fn report(&self) {
        println!("Total Runs: {}", self.total_runs);
        println!("Faults Detected: {}", self.faults_detected);
        println!("Incorrect Results: {}", self.incorrect_results);
        println!("Correct Results: {}", self.correct_results);
    }
}

fn run_fault_injection_cycle() {
    let variables = vec![
        "var1".to_string(),
        "var2".to_string(),
        "var3".to_string(),
    ];
    let num_faults = 1000;
    let faults = generate_faults(variables.clone(), num_faults);

    let mut analyzer = Analyzer::new();

    for fault in faults {
        // Simula il programma con iniezione
        let mut arr = vec![
            RedundantVar::new(5),
            RedundantVar::new(4),
            RedundantVar::new(3),
            RedundantVar::new(2),
            RedundantVar::new(1),
        ];
        let mut is_fault_detected = false;

        if let Some(var_name) = variables.iter().find(|&v| *v == fault.variable_name) {
            // Trova la variabile e applica il fault
            inject_fault(&mut arr[0], fault.bit_to_flip.into()); // Cambiare indice in base alla mappatura
            is_fault_detected = !arr[0].is_valid();
        }

        // Esegui il programma
        let result = std::panic::catch_unwind(|| {
            bubble_sort(&mut arr);
            arr.into_iter().map(|x| x.get()).collect::<Vec<_>>()
        });
        // Analizza il risultato
        let result = result.ok();
        analyzer.analyze_run(result, is_fault_detected);
    }

    // Riporta i risultati
    analyzer.report();
}

fn main() {
    println!("Avvio del ciclo di iniezione fault:");
    run_fault_injection_cycle();
}
