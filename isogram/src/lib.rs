use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    // unimplemented!("Is {} an isogram?", candidate);
    let mut h = HashSet::<char>::new();
    for c in candidate
        .chars()
        .filter(|x| x.is_alphabetic())
        .map(|c| c.to_ascii_lowercase())
    {
        if h.contains(&c) {
            return false;
        }
        h.insert(c);
    }
    true
}
