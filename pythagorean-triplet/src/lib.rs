use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    // unimplemented!("Given the sum {}, return all possible Pythagorean triplets, which produce the said sum, or an empty HashSet if there are no such triplets. Note that you are expected to return triplets in [a, b, c] order, where a < b < c", sum);
    // 1
    // a+b+c = sum
    // 2
    // a^2+b^2 = c^2，短边平方和等于长边平方
    // 3
    // 能构成三角形

    // eg.
    // 3 + 4 + 5 = 12

    // 假设c最长，则先确定c，然后a、b用双指针

    // 0<a<=b<=c

    let mut ans = HashSet::<[u32; 3]>::new();

    for c in (1..=sum - 2).rev() {
        // println!("c::{c}");
        if c <= 0 {
            continue;
        }
        // a+b<=c不能能构成三角形
        if sum - c <= c {
            continue;
        }
        for b in (1..sum - c).rev() {
            // println!("b::{b}");
            if b > c {
                continue;
            }
            if b <= 0 {
                continue;
            }
            let a = sum - c - b;
            if a > b {
                break;
            }

            // println!("a::{a}");

            // check ans
            if a * a + b * b == c * c {
                // println!("yes::a,b,c::{a},{b},{c}");
                ans.insert([a, b, c]);
            }
        }
    }
    ans
}
