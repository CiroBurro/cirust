use crate::utils::argomento::nuovo_argomento;
use crate::utils::argomento::Argomento;
use crate::utils::materia::Materie;
use cursive::views::{Button, Dialog, DummyView, LinearLayout, SelectView, TextView};
use cursive::Cursive;
use cursive::Vec2;

pub fn tui() {
    let mut siv = cursive::default();

    let bottoni = LinearLayout::vertical()
        .child(Button::new("Aggiungi argomento", aggiungi_arg))
        .child(Button::new("Visualizza argomenti", visualizza_arg))
        .child(Button::new("Gestione verifiche", gestione_verifiche))
        .child(DummyView)
        .child(DummyView)
        .child(Button::new("Esci", Cursive::quit));

    let testo:String = String::from("School Organizer è un programma per gestire il proprio studio. \nPermette di avere un resoconto del proprio anno scolastico tramite gli argomenti svolti e i collegamenti tra di essi. \nInoltre aiuta l'organizzazione con il suo sistema di gestione verifiche.");

    siv.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(TextView::new(testo))
                .child(DummyView)
                .child(bottoni),
        )
        .title("School Organizer"),
    );

    siv.run();
}

pub fn aggiungi_arg(s: &mut Cursive) {}
pub fn visualizza_arg(s: &mut Cursive) {}
pub fn gestione_verifiche(s: &mut Cursive) {}
