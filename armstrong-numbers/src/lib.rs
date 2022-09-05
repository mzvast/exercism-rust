pub fn is_armstrong_number(num: u32) -> bool {
    // unimplemented!("true if {} is an armstrong number", num)
    // count digits
    let mut v = num;
    let mut digits = 0;
    while v > 0 {
        v /= 10;
        digits += 1;
    }
    // calculate sum
    let mut t = 0;
    let mut cur = 0;
    v = num;
    while v > 0 {
        cur = v % 10;
        t += cur.pow(digits);
        v /= 10;
    }
    t == num
}