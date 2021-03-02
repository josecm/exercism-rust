use std::collections::HashSet;
//use std::collections::HashMap;

//fn anagrams(word1: &str, word2: &str) -> bool {
//    let word1 = word1.to_lowercase();
//    let word2 = word2.to_lowercase();
//
//    let mut word1_sorted: Vec<char> = word1.chars().collect();
//    let mut word2_sorted: Vec<char> = word2.chars().collect();
//
//    word1_sorted.sort_unstable();
//    word2_sorted.sort_unstable();
//
//    word1 != word2 && word1_sorted == word2_sorted
//}
//
//fn anagrams(word1: &str, word2: &str) -> bool {
//    let word1 = word1.to_lowercase();
//    let word2 = word2.to_lowercase();
//
//    let freqs = |w: &str| {
//        w.chars()
//            .map(|c| (c, w.matches(c).count()))
//            .collect::<HashMap<char, usize>>()
//    };
//
//    word1 != word2 && freqs(&word1) == freqs(&word2)
//}

fn anagrams(word1: &str, word2: &str) -> bool {
    let word1 = word1.to_lowercase();
    let word2 = word2.to_lowercase();

    word1 != word2
        && word1.len() == word2.len()
        && word1
            .chars()
            .all(|c| word1.matches(c).count() == word2.matches(c).count())
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    possible_anagrams
        .iter()
        .filter(|&a| anagrams(a, word))
        .map(|&a| a)
        .collect()
}
