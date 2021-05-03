use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {

    if !"ACGT".contains(nucleotide) {
        return Err(nucleotide);
    }

    if let Some(nucleotide) = dna.chars().filter(|&c| !"ACGT".contains(c)).next() {
        return Err(nucleotide);
    }

    Ok(dna.chars().filter(|&c| c == nucleotide).count())
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut dict = HashMap::new();
    for nucleotide in "ACGT".chars() {
        *dict.entry(nucleotide).or_default() = count(nucleotide, dna)?;
    }
    Ok(dict)
}
