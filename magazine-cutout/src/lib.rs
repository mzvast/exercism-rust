// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;
use std::str::SplitWhitespace;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    // 将magazine的内容放入HashMap
    let mut h: HashMap<&str, i32> = HashMap::new();
    for &x in magazine {
        let mut cur = h.entry(x).or_insert(0);
        *cur += 1;
    }
    // 遍历note，确认都在HashMap中，且数量足够
    for &x in note {
        let mut cur = h.entry(x).or_insert(0);
        *cur -= 1;
        if *cur < 0 {
            return false;
        }
    }
    true
}
