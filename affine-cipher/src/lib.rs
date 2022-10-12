/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}
const M: i32 = 26;
const OFFSET: i32 = 97;

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    // unimplemented!("Encode {} with the key ({}, {})", plaintext, a, b);
    // E(x) = (ax + b) mod m

    let encode_char = |c: char, a: i32, b: i32| {
        match c {
            '0'..='9' => c, // 数字不处理
            _ => {
                // println!("c::{}", c as u8);
                let idx = (a * (c as i32 - OFFSET) + b).rem_euclid(M) + OFFSET;
                char::from(idx as u8)
            }
        }
    };

    match get_mmi(a, M) {
        Some(_) => {
            let ans = plaintext
                .to_lowercase() // 统一成小写
                .chars()
                .filter(|&x| x.is_alphanumeric())
                .map(|c| encode_char(c, a, b))
                .collect::<Vec<char>>()
                .chunks(5) // 5个1组
                .map(|chunk| chunk.iter().collect::<String>())
                .collect::<Vec<String>>()
                .join(" ");

            Ok(ans)
        }
        None => Err(AffineCipherError::NotCoprime(a)),
    }
}

// https://www.geeksforgeeks.org/multiplicative-inverse-under-modulo-m/
// https://exercism.org/tracks/rust/exercises/affine-cipher
fn get_mmi(a: i32, m: i32) -> Option<i32> {
    (1..m).find(|n| a * n % m == 1)
}
/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    // unimplemented!("Decode {} with the key ({}, {})", ciphertext, a, b);

    let decode_char = |c: char, a_mmi: i32, b: i32| {
        match c {
            '0'..='9' => c, // 数字不处理
            _ => {
                let idx = (a_mmi * (c as i32 - b - OFFSET)).rem_euclid(M) + OFFSET;
                char::from(idx as u8)
            }
        }
    };

    match get_mmi(a, M) {
        Some(a_mmi) => {
            let ans = ciphertext
                .chars()
                .filter(|&x| x.is_alphanumeric())
                .map(|c| decode_char(c, a_mmi, b))
                .collect::<String>();
            Ok(ans)
        }
        None => Err(AffineCipherError::NotCoprime(a)),
    }
}
