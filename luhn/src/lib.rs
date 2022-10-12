/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    // unimplemented!("Is the Luhn checksum for {} valid?", code);

    // 输入中允许有空格，但在检查之前应该去掉空格
    let mut data = code.replace(" ", "").to_owned();
    // 长度为 1 或更短的字符串无效
    if data.len() <= 1 {
        println!("长度为 1 或更短的字符串无效");
        return false;
    }
    // 不允许使用所有其他非数字字符。
    if data.chars().any(|c| !c.is_ascii_digit()) {
        println!("不允许使用所有其他非数字字符。");
        return false;
    }

    // 从右往左每个第二位加倍，超过9则-9
    let total = data.chars().rev().enumerate().fold(0, |acc, (cnt, cur)| {
        let mut ans = cur.to_digit(10).unwrap();
        if (cnt + 1) % 2 == 0 {
            ans *= 2;
            if ans > 9 {
                ans -= 9;
            }
        }
        return acc + ans;
    });

    // println!("total::{total}");

    if total % 10 != 0 {
        println!("total不对");
        return false;
    }

    true
}
