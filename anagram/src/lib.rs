use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    // unimplemented!(
    //     "For the '{}' word find anagrams among the following words: {:?}",
    //     word,
    //     possible_anagrams
    // );

    let char_to_lowercase = |c: char| {
        if c.is_lowercase() {
            c.to_string()
        } else {
            c.to_lowercase().to_string()
        }
    };

    let cur = word.chars().map(char_to_lowercase).collect::<Vec<_>>();
    let mut cur_sorted = cur.clone();
    cur_sorted.sort_unstable(); // 排序


    let ans = possible_anagrams
        .iter()
        .filter(|w| {
            let mut item: Vec<_> = w.chars().map(char_to_lowercase).collect();
            let is_same_word = item == cur;
            item.sort_unstable();
            cur_sorted == item && !is_same_word
        })
        .map(|&x| x);

    HashSet::from_iter(ans)
}

// fn main() {
//     // println!("{:?}", get_cnt_result("aAbB"));
//     println!("{:?}", "ΑΒΓ".escape_unicode());
//     println!("{:?}", "γβα".escape_unicode());
// }
