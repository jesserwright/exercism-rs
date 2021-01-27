pub fn raindrops(n: u32) -> String {
    let factors = (1..n + 1)
        .into_iter()
        .filter(|x| n % x == 0)
        .collect::<Vec<u32>>();

    let mut sound = String::new();
    if factors.contains(&3) {
        sound.push_str("Pling");
    }
    if factors.contains(&5) {
        sound.push_str("Plang");
    }
    if factors.contains(&7) {
        sound.push_str("Plong");
    }
    if sound.len() == 0 {
        return n.to_string();
    }
    sound
}
