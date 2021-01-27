pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }
    if list.len() == 1 {
        return and_all_for(list[0]);
    }
    list.iter()
        .enumerate()
        .map(|(i, x)| {
            if i == list.len() - 1 {
                return and_all_for(list[0]);
            }
            for_the_want(x, list[i + 1])
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn and_all_for(word: &str) -> String {
    format!("And all for the want of a {}.", word)
}
fn for_the_want(word_1: &str, word_2: &str) -> String {
    format!("For want of a {} the {} was lost.", word_1, word_2)
}
