pub fn reply(message: &str) -> &str {
    let is_question = message.chars().filter(|&c| !c.is_whitespace()).last() == Some('?');
    let has_letters = message.chars().any(|c| c.is_alphabetic());
    let is_uppercase = message
        .chars()
        .filter(|c| c.is_alphabetic())
        .all(|c| c.is_uppercase())
        && has_letters;
    if is_question && is_uppercase {
        return "Calm down, I know what I'm doing!";
    }
    if is_question {
        return "Sure.";
    }
    if is_uppercase {
        return "Whoa, chill out!";
    }
    let trimmed: String = message.chars().filter(|&c| !c.is_whitespace()).collect();
    let is_empty = trimmed.is_empty();
    if is_empty {
        return "Fine. Be that way!";
    }
    "Whatever."
}
