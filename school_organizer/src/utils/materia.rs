use crate::utils::argomento::Argomento;

#[derive(Debug)]
pub struct Materie {
    pub arte: Vec<Argomento>,
    pub filosofia: Vec<Argomento>,
    pub fisica: Vec<Argomento>,
    pub inglese: Vec<Argomento>,
    pub italiano: Vec<Argomento>,
    pub latino: Vec<Argomento>,
    pub matematica: Vec<Argomento>,
    pub scienze: Vec<Argomento>,
    pub storia: Vec<Argomento>,
}

impl Materie {
    pub fn crea_materie() -> Materie {
        Materie {
            arte: Vec::new(),
            filosofia: Vec::new(),
            fisica: Vec::new(),
            inglese: Vec::new(),
            italiano: Vec::new(),
            latino: Vec::new(),
            matematica: Vec::new(),
            scienze: Vec::new(),
            storia: Vec::new(),
        }
    }

    pub fn aggiungi_argomento(&mut self, argomento: Argomento, materia: &str) {
        match materia {
            "arte" => self.arte.push(argomento),
            "filosofia" => self.filosofia.push(argomento),
            "fisica" => self.fisica.push(argomento),
            "inglese" => self.inglese.push(argomento),
            "italiano" => self.italiano.push(argomento),
            "latino" => self.latino.push(argomento),
            "matematica" => self.matematica.push(argomento),
            "scienze" => self.scienze.push(argomento),
            "storia" => self.storia.push(argomento),
            _ => println!("Materia non riconosciuta"),
        }
    }

}