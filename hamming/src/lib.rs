/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    // unimplemented!("What is the Hamming Distance between {} and {}", s1, s2);

    if s1.len() != s2.len() {
        return None;
    }

    let mut ans = 0;
    for i in 0..s1.len() {
        if s1.chars().nth(i).unwrap() != s2.chars().nth(i).unwrap() {
            ans += 1;
        }
    }
    Some(ans)
}
