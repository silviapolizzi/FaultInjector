# GROUP 27
# **Sistema di Fault-Injection per Applicazione Ridondata**

## **Descrizione**
Questo progetto implementa un sistema di fault injection per valutare la resilienza di un'applicazione che utilizza variabili ridondate. Il sistema introduce guasti di tipo bit-flip in un array di numeri interi e analizza il loro effetto sull'algoritmo di Bubble Sort, misurando la capacità del meccanismo di ridondanza nel rilevare e gestire gli errori.
L'applicazione consente di:
- Iniettare **fault casuali** in un array ridondato.
- Eseguire il **Bubble Sort** su dati potenzialmente corrotti.
- Misurare l'**overhead di memoria** e **CPU time** introdotti dalla ridondanza.
- Analizzare il numero di correct runs, fault detected e incorrect runs per valutare l'impatto dei guasti e il comportamento del sistema in presenza di errori

## **Struttura del progetto**
- **`src/Redundant.rs`** → Definizione della struttura dati `Redundant<T>` per la gestione delle variabili ridondate.
- **`src/Sorting.rs`** → Implementazione del **Bubble Sort** con e senza ridondanza.
- **`src/Fault.rs`** → Generazione e iniezione dei guasti.
- **`src/Analyzer.rs`** → Registrazione e analisi dei risultati.
- **`src/Utility.rs`** → Caricamento della configurazione e generazione degli array.
- **`src/main.rs`** → Coordinamento delle fasi del test (configurazione, iniezione guasti, sorting, analisi risultati).

## **Requisiti**
- **Libreria `simplelog`** per la registrazione dei log

## **Installazione e Utilizzo**
### **1. Clonare la repository**
```bash
git clone https://github.com/ProgrammazioneDiSistema2024-IA-ZZ/Group-27.git
cd Group-27
```

### **2. Configurare i parametri**
Modificare il file `config.toml` per impostare i parametri di test:
```toml
num_elements = 500
num_faults = 5000
max_bit_to_flip = 8
fault_injection_time = 10
```
Dove fault_injection_time è in ms.

### **3. Compilare ed eseguire il progetto**
```bash
cargo build
cargo run
```

## **Output e Report**
Dopo l'esecuzione, i risultati saranno disponibili nei file di output generati nella directory **`results/`**:
- **`fault_injection.log`** → Log dettagliati dei test.
- **`analyzer_report.txt`** → Statistiche sui fault rilevati e sull'efficacia della ridondanza.
- **`memory_overhead_report.txt`** → Overhead di memoria.
- **`cpu_time_overhead_report.txt`** → Overhead computazionale.


---

