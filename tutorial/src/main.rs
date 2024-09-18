//tutorial rust

fn main() {
    variabili();
    tipi_dati();
    operazioni();
    let risultato = somma(5, 5);
    println!("Il risultato della somma è {}", risultato);
    control_flow();
    println!("Tutorial completato con successo!");
}

fn variabili() {
    //variabili e costanti
    let a = "a"; // variabile immutabile
    let a = "ab"; // shadowing della variabile a --> non è la stessa variabile, è una nuova variabile con lo stesso nome, quella di prima viene quindi sovrascritta
    let mut b = "b"; // variabile mutabile
    b = "bc"; //variabile b mutata
    const C: &str = "c"; // costante
}

fn tipi_dati() {
    //tipi di dati
    let x: i32= -5; //i32 è un tipo di dato intero con segno, default per gli int è i32
    let y: u32 = 10; //u32 è un tipo di dato intero senza segno, default per gli int è i32
    let z: f32 = 10.5; //f32 è un tipo di dato a virgola mobile, default per i float è f64
    let t: bool = true; //bool è un tipo di dato booleano, scritto esplicitamente, normalmente sarebbe let t = true;
    let c: char = 'c'; //char è un tipo di dato carattere, scritto esplicitamente, normalmente sarebbe let c = 'c'; N.B. char è un tipo di dato a 4 byte, quindi può contenere un solo carattere ed è contenuto tra apici singoli
}

fn tuple_array() {
    //tuple e array
    let tup: (i32, f64, char) = (500, 6.4, 'c'); //tupla di 3 elementi, le tuple possono contenere tipi di dati diversi
    let (x, y, z) = tup; //x, y, z sono variabili che prendono i valori della tupla
    let x = tup.0; //x prende il valore della prima posizione della tupla
    let y = tup.1; //y prende il valore della seconda posizione della tupla
    let z = tup.2; //z prende il valore della terza posizione della tupla
    
    let a = [1, 2, 3, 4, 5]; //array di 5 elementi, implicito
    let b: [i32; 5] = [1, 2, 3, 4, 5]; //array di 5 elementi, esplicito, vengono specificati il tipo di dato e il numero di elementi
    let c = [1; 5]; //array di 5 elementi tutti con il valore 1, sarebbe come fare let c = [1, 1, 1, 1, 1]

    let primo = a[0]; //primo elemento dell'array
    let secondo = a[1]; //secondo elemento dell'array
    let terzo = a[2]; //terzo elemento dell'array
}

fn operazioni() {
    //operazioni
    let somma = 5 + 10;
    let sottrazione = 10 - 5;
    let moltiplicazione = 5 * 10;
    let divisione = 10 / 5;
    let resto = 10 % 3;
    let uguale = 10 == 10;
    let diverso = 10 != 10;
    let maggiore = 10 > 5;
    let minore = 10 < 5;
    let maggiore_uguale = 10 >= 10;
    let minore_uguale = 10 <= 10;
    let and = true && false;
    let or = true || false;
    let not = !true;
}

fn somma (a: i32, b: i32) -> i32{
    a + b
}


fn control_flow() {
    //condizioni
    let x: i8 = 10;
    if x > 5 {
        println!("x è maggiore di 5");
    } else if x == 5 {
        println!("x è uguale a 5");
    } else {
        println!("x è minore di 5");
    }

    let condizione: bool = true;
    let numero = if condizione {5} else {6}; //se la condizione è vera la variabile numero assume il valore di 5, se è falsa il valore sarà 6

    //loops

    //loop --> loop infinito
    let mut counter: i32 = 0; 
    loop {
        counter += 1;
        println!("{}", counter);
        if counter >= 10 {
            break //keyword break esce dal loop
        }
    }


    
}