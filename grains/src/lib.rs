pub fn square(s: u32) -> u64 {
    assert!(s > 0 && s <= 64, "Square must be between 1 and 64");
    (1..s).fold(1, |acc, _| acc * 2)
}

pub fn total() -> u64 {
    u64::max_value()
}
