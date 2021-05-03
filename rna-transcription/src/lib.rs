use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Dna(String);

#[derive(Debug, PartialEq)]
pub struct Rna(String);

impl Dna {
    const NUCLEOTIDES: &'static str = "ACGT";

    pub fn new(dna: &str) -> Result<Dna, usize> {
        match dna.chars().position(|c| !Dna::NUCLEOTIDES.contains(c)) {
            None => Ok(Dna(String::from(dna))),
            Some(n) => Err(n),
        }
    }

    pub fn into_rna(self) -> Rna {
        let map: HashMap<char, char> = vec![('G', 'C'), ('C', 'G'), ('T', 'A'), ('A', 'U')]
            .into_iter()
            .collect();
        Rna(self.0.chars().map(|c| map[&c]).collect())
    }
}

impl Rna {
    const NUCLEOTIDES: &'static str = "CGAU";
    pub fn new(rna: &str) -> Result<Rna, usize> {
        match rna.chars().position(|c| !Rna::NUCLEOTIDES.contains(c)) {
            None => Ok(Rna(String::from(rna))),
            Some(n) => Err(n),
        }
    }
}
