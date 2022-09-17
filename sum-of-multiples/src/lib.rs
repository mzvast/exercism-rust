pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    // unimplemented!(
    //     "Sum the multiples of all of {:?} which are less than {}",
    //     factors,
    //     limit
    // )
    (1..limit)
        .filter(|x| factors.iter().any(|&f| f != 0 && x % f == 0))
        .sum()
}
