use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    // unimplemented!("Count of occurrences of words in {:?}", words);
    let mut h = HashMap::<String, u32>::new();
    for x in words
        .to_lowercase()
        .split(|ch: char| ch != '\'' && !ch.is_ascii_alphanumeric())
        .filter(|w| !w.is_empty())
    {
        let cur = x.trim_matches('\''); //去掉首尾的引号
        let item = h.entry(cur.to_string()).or_insert(0);
        *item += 1;
    }
    h
}
