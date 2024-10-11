use crate::utils::argomento::nuovo_argomento;
use crate::utils::argomento::Argomento;
use crate::utils::materia::Materie;
use cursive::view::Nameable;
use cursive::views::{Button, Dialog, DummyView, EditView, LinearLayout, TextView};
use cursive::{Cursive, Vec2};
use cursive::view::Resizable;



pub fn tui() {
    let mut siv = cursive::default();

    let bottoni = LinearLayout::vertical()
        .child(DummyView)
        .child(Button::new("Aggiungi argomento", aggiungi_arg))
        .child(DummyView)
        .child(Button::new("Visualizza argomenti", visualizza_arg))
        .child(DummyView)
        .child(Button::new("Gestione verifiche", gestione_verifiche))
        .child(DummyView.fixed_height(20))
        .child(Button::new("Esci", Cursive::quit));

    let testo:String = String::from("School Organizer è un programma per gestire il proprio studio. \nPermette di avere un resoconto del proprio anno scolastico tramite gli argomenti svolti e i collegamenti tra di essi. \nInoltre aiuta l'organizzazione con il suo sistema di gestione verifiche.");
    
    let ascii_art = r#"   _____      __                __               
  / ___/_____/ /_  ____  ____  / /               
  \__ \/ ___/ __ \/ __ \/ __ \/ /                
 ___/ / /__/ / / / /_/ / /_/ / /                 
/____/\___/_/ /_/\____/\____/_/ _                
  / __ \_________ _____ _____  (_)___  ___  _____
 / / / / ___/ __ `/ __ `/ __ \/ /_  / / _ \/ ___/
/ /_/ / /  / /_/ / /_/ / / / / / / /_/  __/ /    
\____/_/   \__, /\__,_/_/ /_/_/ /___/\___/_/     
          /____/                                 "#;


    let left_content = LinearLayout::vertical().child(DummyView).child(TextView::new(testo)).child(DummyView.fixed_height(5)).child(TextView::new(ascii_art));

    siv.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(left_content)
                .child(DummyView.fixed_width(10))
                .child(bottoni),
        )
        .title("School Organizer")
        .fixed_size(Vec2::new(100, 30))
    );

    siv.run();
}

pub fn aggiungi_arg(s: &mut Cursive){
    
    let titolo_arg = EditView::new().fixed_width(20).with_name("titolo_arg");
    let materia_arg = EditView::new().fixed_width(20).with_name("materia_arg");
    let descrizione_arg = EditView::new().fixed_width(20).with_name("descrizione_arg");


    let left_content = LinearLayout::vertical().child(TextView::new("Titolo: ")).child(DummyView).child(TextView::new("Materia: ")).child(DummyView).child(TextView::new("Descrizione: "));
    let right_content = LinearLayout::vertical().child(titolo_arg).child(DummyView).child(materia_arg).child(DummyView).child(descrizione_arg);

    s.add_layer(Dialog::around(
        LinearLayout::horizontal()
            .child(left_content)
            .child(right_content)
    )
    .title("Inserisci nuovo argomento")
    .button("Ok", move |s| {
        let mut materie = Materie::crea_materie();

        let titolo = s.call_on_name("titolo_arg", |view: &mut EditView| {view.get_content()}).unwrap().to_string();
        let materia = s.call_on_name("materia_arg", |view: &mut EditView| {view.get_content()}).unwrap().to_string();
        let descrizione = s.call_on_name("descrizione_arg", |view: &mut EditView| {view.get_content()}).unwrap().to_string();

        let arg:Argomento = nuovo_argomento(titolo, descrizione);
        materie.aggiungi_argomento(arg, &materia);
    })
    .button("Annulla", |s| {
        s.pop_layer();
    }));


}
pub fn visualizza_arg(_s: &mut Cursive) {}
pub fn gestione_verifiche(_s: &mut Cursive) {}


