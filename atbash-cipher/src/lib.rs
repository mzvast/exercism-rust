/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    // unimplemented!("Encoding of {plain:?} in Atbash cipher.");
    let mut cnt = 0;
    plain
        .bytes()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| {
            cnt += 1;
            let mut encoded;
            if c.is_ascii_digit() {
                encoded = (c as char).to_string(); // 非字母
            } else {
                let d = (219 - c.to_ascii_lowercase()) as char;
                encoded = d.to_string();
            }

            if cnt % 5 == 1 && cnt != 1 {
                // 每5个加1个空格
                encoded = " ".to_string() + &encoded;
            }
            encoded
        })
        .collect()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    // unimplemented!("Decoding of {cipher:?} in Atbash cipher.");
    encode(cipher).chars().filter(|&c| c != ' ').collect()
}

// fn main() {
//     // println!("{}", encode("gsvjf rxpyi ldmul cqfnk hlevi gsvoz abwlt"));
//     println!("{}", decode("zmlyhgzxovrhlugvmzhgvkkrmthglmv"));
// }
