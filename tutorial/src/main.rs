//tutorial rust

fn main() {
    variabili();
    tipi_dati();
    operazioni();
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