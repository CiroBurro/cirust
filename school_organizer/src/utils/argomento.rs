
#[derive(Debug)]
pub struct Argomento{
    pub titolo: String,
    pub descrizione: String,
    pub collegamenti: Vec<Argomento>
}

pub fn nuovo_argomento(titolo: String, descrizione: String ) -> Argomento{
    Argomento {
        titolo,
        descrizione,
        collegamenti: Vec::new()
        
    }
}



