use std::collections::VecDeque;

// "WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWB"  ->  "12WB12W3B24WB"
// "AABCCCDEEEE"  ->  "2AB3CD4E"
pub fn encode(source: &str) -> String {
    // unimplemented!("Return the run-length encoding of {}.", source);
    let mut pre: char = '!';
    let mut cnt = 0 as u8;
    let mut ans = String::new();
    for x in source.chars().chain("$".chars()) {
        // 后面补一个终止符号
        if x == pre {
            cnt += 1;
        } else {
            // new word or ended
            if pre != '!' {
                // 非第一个
                if cnt > 1 {
                    ans.push_str(cnt.to_string().as_str());
                    ans.push(pre);
                } else {
                    ans.push(pre);
                }
            }

            pre = x;
            cnt = 1;
        }
    }
    ans
}

// "2AB3CD4E"  ->  "AABCCCDEEEE"
pub fn decode(source: &str) -> String {
    // unimplemented!("Return the run-length decoding of {}.", source);
    let mut ans = String::new();

    let mut cnt_total: u32 = 0; // 当前字母出现的次数
    for cur in source.chars().chain("$".chars()) {
        if cur.is_ascii_alphabetic() || cur.is_ascii_whitespace() || cur == '$' {
            if cur != '$' {
                ans.push_str(
                    cur.to_string()
                        .repeat(if cnt_total == 0 {
                            1 // 默认一次
                        } else {
                            cnt_total as usize
                        })
                        .as_str(),
                );
            }
            cnt_total = 0;
        }

        if cur.is_numeric() {
            cnt_total = cnt_total * 10 + cur.to_digit(10).unwrap();
        }
    }

    ans
}
