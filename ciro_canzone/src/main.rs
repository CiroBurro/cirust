fn main() {
    testo_canzone();
}

fn testo_canzone() {
    let testo = [
        "Oh ciro cirooooo",
        "Mangia burro e cantaaa",
        "Ogni morso è una festa",
        "Ogni boccone è una danza",
        "Ciro cirooooo",
        "Con il burro nel cuoreeee",
        "La vita è più dolce, è un tripudio di saporeee",
    ];

    for rima in testo.iter() {
        println!("{}", rima);
    }
}