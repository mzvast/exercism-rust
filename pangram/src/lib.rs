use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    // unimplemented!("Is {} a pangram?", sentence);
    let mut letters = vec![false; 26];

    sentence
        .to_lowercase()
        .chars()
        .filter(|x| x.is_ascii_alphabetic())
        .collect::<HashSet<char>>()
        .len()
        == 26
}
