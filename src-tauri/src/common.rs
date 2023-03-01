pub fn rem_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();

    chars.next();
    chars.next_back();

    return chars.as_str();
}
