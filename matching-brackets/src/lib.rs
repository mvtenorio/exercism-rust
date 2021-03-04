pub fn brackets_are_balanced(string: &str) -> bool {
    let brackets = string.chars().filter(|c| "{[()]}".contains(*c));
    let mut to_be_closed = vec![];

    for bracket in brackets {
        match (bracket, to_be_closed.last()) {
            ('{', _) => to_be_closed.push('}'),
            ('[', _) => to_be_closed.push(']'),
            ('(', _) => to_be_closed.push(')'),
            (_, Some(last_bracket)) if &bracket == last_bracket => {
                to_be_closed.pop();
            }
            _ => return false,
        }
    }

    to_be_closed.len() == 0
}
