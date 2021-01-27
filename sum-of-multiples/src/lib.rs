pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|x| {
            factors.iter().any(|factor| {
                if factor == &0 {
                    return false;
                }
                x % factor == 0
            })
        })
        .sum()
}
