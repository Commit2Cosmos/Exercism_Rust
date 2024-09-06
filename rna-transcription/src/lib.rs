use std::collections::HashMap;
use lazy_static::lazy_static;

#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

lazy_static! {
    static ref dna_to_rna: HashMap<char, char> = {
        HashMap::from([
            ('G', 'C'),
            ('C', 'G'),
            ('T', 'A'),
            ('A', 'U'),
        ])
    };

    static ref rna_to_dna: HashMap<char, char> = {
        HashMap::from([
            ('C', 'G'),
            ('G', 'C'),
            ('U', 'A'),
            ('A', 'T'),
        ])
    };
}

impl Dna {
    
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let first_incorrect = dna.chars().position(|c| !dna_to_rna.contains_key(&c));
        if let Some(k) = first_incorrect {
            Err(k)
        } else {
            Ok(Self(dna.to_string()))
        }
    }

    pub fn into_rna(self) -> Rna {
        Rna(self.0.chars().map(|x| dna_to_rna.get(&x).unwrap()).collect())
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let first_incorrect = rna.chars().position(|c| !rna_to_dna.contains_key(&c));
        if let Some(k) = first_incorrect {
            Err(k)
        } else {
            Ok(Self(rna.to_string()))
        }
    }
}
