pub fn square(s: u32) -> u64 {
    // unimplemented!("grains of rice on square {}", s);
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64")
    }
    (2 as u64).pow(s - 1) as u64
}

pub fn total() -> u64 {
    // unimplemented!();
    let mut pre = 1 as u64;
    let mut idx = 0 as u64;
    let mut sum = 0 as u64;
    while idx < 64 {
        sum += pre;
        if idx == 63 {
            break;
            // prevent overflow
        }
        pre *= 2;
        idx += 1;
    }
    sum
}
