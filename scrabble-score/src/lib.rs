/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    // unimplemented!("Score {} in Scrabble.", word);

    word.chars().fold(0, |acc, x| acc + get_score(&x))
}

fn get_score(&x: &char) -> u64 {
    match x.to_ascii_uppercase() {
        'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T' => 1,
        'D' | 'G' => 2,
        'B' | 'C' | 'M' | 'P' => 3,
        'F' | 'H' | 'V' | 'W' | 'Y' => 4,
        'K' => 5,
        'J' | 'X' => 8,
        'Q' | 'Z' => 10,
        _ => 0,
    }
}
