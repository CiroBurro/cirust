use std::io;

fn main() {
    println!("Trova l'ennesimo numero della sequenza di Fibonacci!");
    println!("Inserisci il numero che vuoi trovare: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Errore nella lettura dell'input");
    
    let n:f64 = input
        .trim()
        .parse()
        .expect("Inserisci un numero valido!");


    let  cinque_sqrt:f64 = radice_quadrata(5.0);
    let phi:f64 = (1.0 + cinque_sqrt)/2.0;
    let x:f64 = (phi.powf(n) - (phi - 1.0).powf(n))/cinque_sqrt;
    println!("il numero che cercavi è: {x}");
     
}

fn radice_quadrata(numero:f64) -> f64{
    numero.sqrt()
}