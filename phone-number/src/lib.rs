// (NXX)-NXX-XXXX
// N 2-9
// X 0-9

// +1 (613)-995-0253
// 613-995-0253
// 1 613 995 0253
// 613.995.0253

// 结果都是 6139950253
pub fn number(user_number: &str) -> Option<String> {
    // unimplemented!(
    //     "Given the number entered by user '{}', convert it into SMS-friendly format. If the entered number is not a valid NANP number, return None.",
    //     user_number
    // );
    // 首先去除特殊字符

    let mut temp = user_number
        .chars()
        .filter(|x| x.is_numeric())
        .collect::<String>();

    // 去掉区号
    if temp.len() == 11 && temp.chars().nth(0).unwrap().to_digit(10).unwrap() == 1 {
        temp.remove(0);
    }

    match temp.len() {
        10 => {
            if temp.chars().nth(0).unwrap().to_digit(10).unwrap() < 2
                || temp.chars().nth(3).unwrap().to_digit(10).unwrap() < 2
            {
                return None;
            }
            return Some(temp);
        }
        _ => return None,
    }
}
