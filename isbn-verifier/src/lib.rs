/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    // unimplemented!("Is {:?} a valid ISBN number?", isbn);

    if let Some(idx) = isbn.chars().position(|x| x == 'X') {
        // 有X且不在末尾，非法
        if idx != isbn.len() - 1 {
            return false;
        }
    };


    // 去除横线
    let nums = isbn
        .chars()
        .filter(|x| x.is_numeric() || x == &'X')
        .collect::<Vec<_>>();

    if nums.len() != 10 {
        false
    } else {
        nums.iter().enumerate().fold(0, |acc, (idx, x)| {
            let mut cur = match x {
                &'X' => 10,
                _ => x.to_digit(10).unwrap() as i32,
            };
            return acc + ((idx + 1) as i32 * cur);
        }) % 11
            == 0
    }
}
