pub fn nth(n: u32) -> u32 {
    // unimplemented!("What is the 0-indexed {}th prime number?", n)
    // 除了1和本身没有其他约数
    // 用2到sqrt(n)去除，能除尽则非素数
    let mut idx = 0; // 从0开始
    let mut x = 2;
    while idx <= n {
        if is_prime(x) {
            // println!("{cnt} {}", i);
            if idx == n {
                break;
            }
            idx += 1;
        }
        // todo: 或许可以跳过一些明显的
        x += 1;
    }
    x
}

fn is_prime(n: u32) -> bool {
    for i in 2..=(n as f32).sqrt() as u32 {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}
