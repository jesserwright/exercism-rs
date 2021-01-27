pub fn is_armstrong_number(num: u32) -> bool {
    num.to_string()
        .chars()
        .map(|x| u32::pow(x.to_digit(10).unwrap(), num.to_string().len() as u32))
        .collect::<Vec<_>>()
        .iter()
        .sum::<u32>()
        == num
}
