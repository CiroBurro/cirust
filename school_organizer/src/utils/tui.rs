//use cursive::views::{Button, Dialog, LinearLayout, TextView};
use cursive_tree_view::{TreeView, Placement};
use crate::utils::materia::Materie;
use crate::utils::argomento::nuovo_argomento;
use crate::utils::argomento::Argomento;
use std::collections::HashMap;

pub fn tui() {
    let mut siv = cursive::default();

    let mut materia = Materie::crea_materie();

    let hegel:Argomento = nuovo_argomento("hegel", "descrizione");
    let feuerbach:Argomento = nuovo_argomento("feuerbach", "descrizione");
    materia.aggiungi_argomento(hegel, "filosofia");
    materia.aggiungi_argomento(feuerbach, "filosofia");

    let limiti:Argomento = nuovo_argomento("limiti", "Strumento matematico per determinare come si comporta una funzione in un determinato punto");
    let integrali:Argomento = nuovo_argomento("integrali", "Strumento matematico per determinare come si comporta una funzione in un determinato punto");
    let logaritmi:Argomento = nuovo_argomento("logaritmi", "Strumento matematico per determinare come si comporta una funzione in un determinato punto");
    materia.aggiungi_argomento(limiti, "matematica");
    materia.aggiungi_argomento(integrali, "matematica");
    materia.aggiungi_argomento(logaritmi, "matematica");


    

    let tree = tree_view(materia);

    
    siv.add_layer(tree);

    siv.run();
}


pub fn tree_view(materia: Materie) -> TreeView<String>{
    let mut tree_view: TreeView<String> = TreeView::new();

    // Inserimento delle materie
    let arte_index = tree_view.insert_item("Arte".to_string(), Placement::After, 0);
    let filosofia_index = tree_view.insert_item("Filosofia".to_string(), Placement::After, arte_index.unwrap());
    let fisica_index = tree_view.insert_item("Fisica".to_string(), Placement::After, filosofia_index.unwrap());
    let inglese_index = tree_view.insert_item("Inglese".to_string(), Placement::After, fisica_index.unwrap());
    let italiano_index = tree_view.insert_item("Italiano".to_string(), Placement::After, inglese_index.unwrap());
    let latino_index = tree_view.insert_item("Latino".to_string(), Placement::After, italiano_index.unwrap());
    let matematica_index = tree_view.insert_item("Matematica".to_string(), Placement::After, latino_index.unwrap());
    let scienze_index = tree_view.insert_item("Scienze".to_string(), Placement::After, matematica_index.unwrap());
    let storia_index = tree_view.insert_item("Storia".to_string(), Placement::After, scienze_index.unwrap());

    // Aggiungi gli argomenti come figli delle rispettive materie
    for argomento in materia.arte {
        tree_view.insert_item(argomento.titolo, Placement::LastChild, arte_index.unwrap());
    }

    for argomento in materia.filosofia {
        tree_view.insert_item(argomento.titolo, Placement::LastChild, filosofia_index.unwrap());
    }

    for argomento in materia.fisica {
        tree_view.insert_item(argomento.titolo, Placement::LastChild, fisica_index.unwrap());
    }

    for argomento in materia.inglese {
        tree_view.insert_item(argomento.titolo, Placement::LastChild, inglese_index.unwrap());
    }

    for argomento in materia.italiano {
        tree_view.insert_item(argomento.titolo, Placement::LastChild, italiano_index.unwrap());
    }

    for argomento in materia.latino {
        tree_view.insert_item(argomento.titolo, Placement::LastChild, latino_index.unwrap());
    }

    for argomento in materia.matematica {
        tree_view.insert_item(argomento.titolo, Placement::LastChild, matematica_index.unwrap());
    }

    for argomento in materia.scienze {
        tree_view.insert_item(argomento.titolo, Placement::LastChild, scienze_index.unwrap());
    }

    for argomento in materia.storia {
        tree_view.insert_item(argomento.titolo, Placement::LastChild, storia_index.unwrap());
    }

    tree_view
}


