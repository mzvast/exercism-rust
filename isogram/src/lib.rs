use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    // unimplemented!("Is {} an isogram?", candidate);
    let mut h = HashSet::<char>::new();
    candidate
        .to_lowercase()
        .chars()
        .filter(|x| x.is_alphabetic())
        .all(|c| h.insert(c))
}
