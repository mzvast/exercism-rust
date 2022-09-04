/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        let mut x = value;
        let mut rev: u64 = 0;

        // 如果以0结尾，必须是0
        if x % 10 == 0 && x != 0 {
            return None;
        }

        while x > rev {
            // x的低位放到rev的高位
            rev = rev * 10 + x % 10;
            x /= 10
        }
        // 奇数情况
        // x        rev
        // 12321    0
        // 1232     1
        // 123      12
        // 12       123

        // 偶数情况
        // x        rev
        // 1221    0
        // 122     1
        // 12      12

        if x == rev || (rev / 10 == x) {
            Some(Palindrome(value))
        } else {
            None
        }
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    // unimplemented!(
    //     "returns the minimum and maximum number of palindromes of the products of two factors in the range {} to {}",
    //     min, max
    // );
    let mut min_v: Option<Palindrome> = None;
    let mut max_v: Option<Palindrome> = None;
    for i in min..=max {
        for j in min..max {
            if let Some(x) = Palindrome::new(i * j) {
                min_v = if min_v.is_some() {
                    std::cmp::min(Some(x), min_v)
                } else {
                    Some(x)
                };

                max_v = if max_v.is_some(){
                    std::cmp::max(Some(x),max_v)
                } else{
                    Some(x)
                }
            }
        }
    }

    if min_v.is_none()||max_v.is_none() {
        None
    }else{
        Some((min_v.unwrap(),max_v.unwrap()))
    }
    // dbg!(&v[0]);
    // if v.is_empty() {
    //     None
    // } else {
    //     Some((*v.iter().min().unwrap(), *v.iter().max().unwrap()))
    // }
}
