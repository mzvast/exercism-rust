use std::{borrow::BorrowMut, collections::HashMap};

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    // unimplemented!(
    //     "How much of nucleotide type '{}' is contained inside DNA string '{}'?",
    //     nucleotide,
    //     dna
    // );

    let is_invalid = |x| match x {
        'A' | 'T' | 'C' | 'G' => false,
        _ => true,
    };
    let has_invalid_nucleotide = is_invalid(nucleotide);
    let has_invalid_words = dna.chars().any(is_invalid);
    if has_invalid_nucleotide || has_invalid_words {
        return Err('X');
    }
    Ok(dna.chars().filter(|&x| x == nucleotide).count())
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    // unimplemented!(
    //     "How much of every nucleotide type is contained inside DNA string '{}'?",
    //     dna
    // );
    let mut ans: HashMap<char, usize> = HashMap::new();
    let all_letters = ['A', 'T', 'C', 'G'];
    for ch in all_letters {
        if let Ok(cnt) = count(ch, dna) {
            ans.insert(ch, cnt);
        } else {
            return Err('X');
        }
    }
    Ok(ans)
}
