pub fn factors(n: u64) -> Vec<u64> {
    // unimplemented!("This should calculate the prime factors of {}", n)
    let mut remain = n;
    let mut ans = Vec::new();
    let mut candidates = 2..;

    while remain > 1 {
        let cur = candidates.next().unwrap();
        while remain % cur == 0 {
            ans.push(cur);
            remain /= cur;
        }
    }

    ans
}
