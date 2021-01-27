pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();
    for c in string.chars() {
        match c {
            ']' | '}' | ')' => {
                // if the prev bracket does not match, fail it.
                if stack.pop() != Some(c) {
                    return false;
                }
            }
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            '(' => stack.push(')'),
            _ => (), // ignore any other char
        };
    }
    stack.is_empty()
}
