use meval::Expr;

fn main() {
    let testo:&str = "(5 + 7*3 - 4)/2";
    let espressione: Expr = testo.parse().unwrap();
    let risultato:f64 = espressione.eval().unwrap();
    println!("{}", risultato)
}
