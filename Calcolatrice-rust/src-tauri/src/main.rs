#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use meval::Expr;

#[tauri::command]
fn risolvi_espressione(testo: &str) -> Result<String, String> {
    
    let espressione: Result<Expr, _> = testo.parse();
    match espressione {
        Ok(expr) => {
            match expr.eval() {
                Ok(risultato) => Ok(format!("{}", risultato)),
                Err(e) => Err(format!("Errore di valutazione: {}", e))
            }
        },
        Err(e) => Err(format!("Errore di parsing: {}", e))
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![risolvi_espressione])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
