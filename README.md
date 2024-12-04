# Group-27


## TO DO

- [x] Tratto per variabile ridondante
- [x] Programma bubble sort
- [x] Fault injector
- [x] Analizzatore
- [x] Funzione che inventa n guasti (almeno >100, tipo 1000). In input prende il n. di guasti e le variabili da iniettare.
- [ ]


## REQUISITI
- [x] Sistema ciclico di iniezione (lancio programma -  lancio iniezione - esito - aggiornamento dei dati)
- [ ] Stima in termini di overhead (memoria e CPU time)

## DA SISTEMARE
- [ ] integrare is_valid in get (TRATTO)
- [ ] sarebbe meglio se all'interno del bubble sort non venisse richiamato l'analyzer, ma magari piuttosto, che quando resituisce false, richiama l'analizer e la funzione log_fault()
- [ ] sempre per rendere il bubble sort independente dal resto del codice, sarebbe meglio che quando fa il get delle variabili, in qualche modo che non ho idea, si interrompese il bubble sort come fa adesso e ritornasse false (non sono sicura si possa fare)

*IN AGGIORNAMENTO*

-----

## Strutture Dati

- **Analyzer**: Gestisce il conteggio dei guasti e i risultati delle operazioni di ordinamento.
- **Redundant<T>**: Una struttura che contiene un valore e la sua copia, consentendo verifiche di validità per garantire l'integrità dei dati.
- **Fault**: Rappresenta un guasto con un indice, il bit da invertire e un ritardo prima dell'iniezione.

## Funzioni

- **bubble_sort**: Ordina un array di valori `Redundant<i32>`, verificando la presenza di guasti durante il processo.
- **fault_injector**: Inietta guasti nell'array a indici specificati dopo un ritardo.
- **generate_faults**: Crea un elenco di guasti casuali da iniettare nell'array.
- **inject_fault**: Inverte un bit specifico nel valore duplicato di una variabile `Redundant`.
- **generate_random_array**: Genera un array di `Redundant<i32>` con valori casuali.


## Barriere: Coordinamento dei Thread

Le barriere sono utilizzate per sincronizzare l'esecuzione dei thread, assicurandosi che ogni thread raggiunga un determinato punto nel programma prima di procedere ulteriormente. In questo caso, le barriere sono utilizzate per sincronizzare l'inizio dell'iniezione di guasti e l'esecuzione del sorting.

Nel codice:

`let start_barrier = Arc::new(Barrier::new(2)); // Barriera per sincronizzazione tra thread`

    `Barrier::new(2)` crea una barriera che richiede che due thread (nel tuo caso, l'iniettore di guasti e il thread di ordinamento) raggiungano la barriera prima di proseguire. Ogni thread che utilizza la barriera deve invocare start_barrier.wait(), che sospende l'esecuzione del thread fino a quando tutti i thread in attesa hanno raggiunto il punto di sincronizzazione.

### Funzionamento della barriera:

    Iniettore di Guasti: Quando il thread dell'iniettore di guasti raggiunge il punto in cui deve avvenire l'iniezione, invoca `start_barrier.wait()`. Questo fa sì che l'iniettore aspetti finché il thread che esegue l'ordinamento non è pronto.

    Thread di Ordinamento (Bubble Sort): Anche il thread che esegue l'ordinamento deve raggiungere `start_barrier.wait()`, quindi sospenderà la sua esecuzione finché l'iniettore non sarà pronto. Dopo che entrambi i thread hanno raggiunto la barriera, possono proseguire insieme, avviando simultaneamente l'iniezione di guasti e l'ordinamento.

Questa sincronizzazione assicura che il guasto venga iniettato esattamente mentre il sorting è in corso.