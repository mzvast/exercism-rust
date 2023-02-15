use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    // unimplemented!("How will you transform the tree {h:?}?")
    let mut ans = BTreeMap::<char, i32>::new();

    for (cnt, val) in h.iter() {
        for c in val.iter() {
            ans.insert((*c).to_ascii_lowercase(), *cnt);
        }
    }

    ans
}
