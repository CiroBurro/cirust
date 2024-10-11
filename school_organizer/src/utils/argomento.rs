
#[derive(Debug)]
pub struct Argomento{
    pub titolo: String,
    pub descrizione: String,
    pub collegamenti: Vec<Argomento>
}

pub fn nuovo_argomento(titolo: &str, descrizione: &str ) -> Argomento{
    Argomento {
        titolo: titolo.to_string(),
        descrizione: descrizione.to_string(),
        collegamenti: Vec::new()
        
    }
}



