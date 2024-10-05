use rand::prelude::IteratorRandom;
use rand::thread_rng;
use std::io;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
enum Carta<'a> {
    Asso {
        nome: &'a str,
        valore: u32,
        seme: &'a str,
    },
    Due {
        nome: &'a str,
        valore: u32,
        seme: &'a str,
    },
    Tre {
        nome: &'a str,
        valore: u32,
        seme: &'a str,
    },
    Quattro {
        nome: &'a str,
        valore: u32,
        seme: &'a str,
    },
    Cinque {
        nome: &'a str,
        valore: u32,
        seme: &'a str,
    },
    Sei {
        nome: &'a str,
        valore: u32,
        seme: &'a str,
    },
    Sette {
        nome: &'a str,
        valore: u32,
        seme: &'a str,
    },
    Otto {
        nome: &'a str,
        valore: u32,
        seme: &'a str,
    },
    Nove {
        nome: &'a str,
        valore: u32,
        seme: &'a str,
    },
    Dieci {
        nome: &'a str,
        valore: u32,
        seme: &'a str,
    },
    Jack {
        nome: &'a str,
        valore: u32,
        seme: &'a str,
    },
    Donna {
        nome: &'a str,
        valore: u32,
        seme: &'a str,
    },
    Re {
        nome: &'a str,
        valore: u32,
        seme: &'a str,
    },
}

fn genera_mazzo() -> Vec<Carta<'static>> {
    const SEMI: [&str; 4] = ["cuori", "quadri", "picche", "fiori"];
    let mut mazzo: Vec<Carta> = Vec::new();
    for seme in SEMI {
        mazzo.push(Carta::Asso {
            nome: "asso",
            valore: 1,
            seme: seme,
        });
        mazzo.push(Carta::Due {
            nome: "due",
            valore: 2,
            seme: seme,
        });
        mazzo.push(Carta::Tre {
            nome: "tre",
            valore: 3,
            seme: seme,
        });
        mazzo.push(Carta::Quattro {
            nome: "quattro",
            valore: 4,
            seme: seme,
        });
        mazzo.push(Carta::Cinque {
            nome: "cinque",
            valore: 5,
            seme: seme,
        });
        mazzo.push(Carta::Sei {
            nome: "sei",
            valore: 6,
            seme: seme,
        });
        mazzo.push(Carta::Sette {
            nome: "sette",
            valore: 7,
            seme: seme,
        });
        mazzo.push(Carta::Otto {
            nome: "otto",
            valore: 8,
            seme: seme,
        });
        mazzo.push(Carta::Nove {
            nome: "nove",
            valore: 9,
            seme: seme,
        });
        mazzo.push(Carta::Dieci {
            nome: "dieci",
            valore: 10,
            seme: seme,
        });
        mazzo.push(Carta::Jack {
            nome: "jack",
            valore: 10,
            seme: seme,
        });
        mazzo.push(Carta::Donna {
            nome: "donna",
            valore: 10,
            seme: seme,
        });
        mazzo.push(Carta::Re {
            nome: "re",
            valore: 10,
            seme: seme,
        });
    }
    mazzo
}

fn pesca_carta(mazzo: &mut Vec<Carta<'static>>, count: &mut i32) {
    let mut generatore = thread_rng();

    if let Some(indice) = (0..mazzo.len()).choose(&mut generatore) {
        let carta = mazzo.remove(indice);
        match carta {
            Carta::Asso { nome, valore, seme } => {
                println!("{} di {}", nome, seme);
                *count -= 1; // Incremento il valore
            }
            Carta::Due { nome, valore, seme } => {
                println!("{} di {}", nome, seme);
                *count += 1; // Decremento il valore
            }
            Carta::Tre { nome, valore, seme } => {
                println!("{} di {}", nome, seme);
                *count += 1;
            }
            Carta::Quattro { nome, valore, seme } => {
                println!("{} di {}", nome, seme);
                *count += 1;
            }
            Carta::Cinque { nome, valore, seme } => {
                println!("{} di {}", nome, seme);
                *count += 1;
            }
            Carta::Sei { nome, valore, seme } => {
                println!("{} di {}", nome, seme);
                *count += 1;
            }
            Carta::Sette { nome, valore, seme } => {
                println!("{} di {}", nome, seme);
            }
            Carta::Otto { nome, valore, seme } => {
                println!("{} di {}", nome, seme);
            }
            Carta::Nove { nome, valore, seme } => {
                println!("{} di {}", nome, seme);
            }
            Carta::Dieci { nome, valore, seme } => {
                println!("{} di {}", nome, seme);
                *count -= 1;
            }
            Carta::Jack { nome, valore, seme } => {
                println!("{} di {}", nome, seme);
                *count -= 1;
            }
            Carta::Donna { nome, valore, seme } => {
                println!("{} di {}", nome, seme);
                *count -= 1;
            }
            Carta::Re { nome, valore, seme } => {
                println!("{} di {}", nome, seme);
                *count -= 1;
            }
        }
    } else {
        println!("Non ci sono più carte nel mazzo");
    }
}

fn main() {
    let time = Duration::new(1, 0);

    println!("Benvenuto nella sezione allenamento dove potrai esercitarti a contare le carte!");
    thread::sleep(time * 2);
    println!("Le regole sono molto semplici: se esce una carta compresa tra il 2 e il 6 assegnale un valore di +1");
    thread::sleep(time * 2);
    println!("Se invece la carta è compresa tra 7 e 9 il suo valore è nullo");
    thread::sleep(time * 2);
    println!("Mentre se è un 10, un asso o una figura il suo valore sarà di -1");
    thread::sleep(time * 2);
    println!("Somma tutti i punteggi e alla fine scoprirai se hai tenuto bene il conto!");

    let mut main = true;

    loop {
        thread::sleep(time * 3);
        println!("La sfida inizia tra:");
        thread::sleep(time);
        println!("3...");
        thread::sleep(time);
        println!("2...");
        thread::sleep(time);
        println!("1...");
        thread::sleep(time);
        println!("0!");
        thread::sleep(time);
        let mut mazzo = genera_mazzo();
        let mut count: i32 = 0;
        for i in 0..30 {
            pesca_carta(&mut mazzo, &mut count);
            thread::sleep(time * 3);
        }
        println!("Il conto finale è di: {}", count);

        println!("Vuoi fare un'altra partita? (s/n)");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Errore nella lettura dell'input");
        if input.trim() == "n" {
            break;
        }
    }
    println!("Spero che l'allenamento sia stato proficuo, arrivederci!")
}
