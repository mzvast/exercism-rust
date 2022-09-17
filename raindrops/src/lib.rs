pub fn raindrops(n: u32) -> String {
    // unimplemented!("what sound does Raindrop #{} make?", n)
    let mut ans = String::new();
    if n % 3 == 0 {
        ans.push_str("Pling");
    }
    if n % 5 == 0 {
        ans.push_str("Plang");
    }
    if n % 7 == 0 {
        ans.push_str("Plong");
    }

    if ans.is_empty() {
        ans.push_str(n.to_string().as_str());
    }
    ans
}
