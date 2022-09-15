use rand::Rng;
pub fn private_key(p: u64) -> u64 {
    // unimplemented!("Pick a private key greater than 1 and less than {}", p)
    rand::thread_rng().gen_range(2..p)
}

// b^e mod m
// https://zh.m.wikipedia.org/zh-hans/%E6%A8%A1%E5%B9%82
// 从右到左二位算法
// base->b
// exponent->e
// modulus->m
fn modular_pow(mut b: u64, mut e: u64, m: u64) -> u64 {
    let mut ans = 1;
    if m == 1 {
        return 0;
    }
    while e > 0 {
        if e % 2 == 1 {
            // 最低位是1
            ans = (ans * b) % m;
        }
        e = e >> 1;
        b = (b * b) % m;
    }
    ans
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    // unimplemented!(
    //     "Calculate public key using prime numbers {} and {}, and private key {}",
    //     p,
    //     g,
    //     a
    // )
    // println!("{}", LIMIT);
    modular_pow(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    // unimplemented!(
    //     "Calculate secret key using prime number {}, public key {}, and private key {}",
    //     p,
    //     b_pub,
    //     a
    // )
    modular_pow(b_pub, a, p)
}
