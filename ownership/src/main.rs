fn main() {
    let s: String = String::from("Hello, world!");  //la variabile s ha l'ownership del valore stringa "Hello, world!"
    take_ownership(s);  //Passando la variabile s come argomento di una funzione, l'ownership passa al parametro stringa, che però non sarà più valido terminata la funzione

    // se ora provo ora a stamprare il valore della variabile s con println!("{s}"); otterrò un errore al momento di compilazione, perché l'ownership di questo valore ce l'ha il parametro stringa, ma non è più valido

    let s1: String = String::from("Ciao, Mondo!");  //la variabile s1 ha l'ownership del valore stringa "Ciao, Mondo!"
    let s1 = take_give_ownership(s1);  //Nuovamente la variabile s1 viene passata come argomento di una variabile, trasferendo l'ownership al parametro stringa, ma questo parametro sta volta viene ritornato...
    println!("Stampa di main: {s1}");                    //... alla fine delle funzione, e salvato nuovamente nella variabile s1, dando indietro l'ownership, di conseguenza posso usare nuovamente la variabile s1 nel codice e stampare il suo valore
    println!("Seconda stampa di main; {s1}");  // posso usare la variabile s1 finché non cambia il proprietario del suo valore
}


fn take_ownership(stringa:String) {
    println!("{stringa}")
    //qui il parametro stringa è valido
} //da qui in poi no

fn take_give_ownership(stringa:String) -> String {
    println!("Stampa della funzione: {stringa}");
    //qui il parametro stringa è valido

    stringa //restituendo il valore contenuto nel parametro stringa posso trasferire l'ownership di questo valore, che quindi rimarrà valido anche dopo il termine della funzione
} //da qui in poi no