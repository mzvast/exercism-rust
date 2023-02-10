#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    // unimplemented!("classify {num}");
    if num == 0 {
        return None;
    }
    let sum: u64 = (1..=num/2).filter(|x | num % x ==0).sum(); // 0.69s
    // for i in 1..=num / 2 { // 0.46s
    //     if num % i == 0 {
    //         sum += i;
    //     }
    // }

    if sum == num {
        Some(Classification::Perfect)
    } else if sum > num {
        Some(Classification::Abundant)
    } else {
        Some(Classification::Deficient)
    }
}
