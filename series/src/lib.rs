pub fn series(digits: &str, len: usize) -> Vec<String> {
    // unimplemented!(
    //     "What are the series of length {} in string {:?}",
    //     len,
    //     digits
    // )
    // let mut ans: Vec<String> = Vec::new();
    if len == 0 {
        return vec!["".to_string(); digits.len() + 1];
    }

    digits
        .chars()
        .collect::<Vec<char>>()
        .windows(len)
        .map(|c| c.into_iter().collect::<String>())
        .collect()
}
