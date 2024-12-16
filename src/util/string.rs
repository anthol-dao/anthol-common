/// Remove all whitespace ([https://www.unicode.org/reports/tr44/#White_Space]) from the string and replace with a single space.
pub fn adjust_whitespaces(s: &str) -> String {
    s.split_whitespace().collect::<Vec<&str>>().join(" ")
}

/// Remove all whitespace ([<https://www.unicode.org/reports/tr44/#White_Space>]) from the string.
pub fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}
