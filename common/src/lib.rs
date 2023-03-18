pub fn is_numeric(s: &str) -> bool {
    s.chars()
        .map(|c| ('0'..='9').contains(&c) || c == '.')
        .fold(true, |acc, c| acc && c)
}

pub fn is_string(s: &str) -> bool {
    s.starts_with('"') && s.ends_with('"')
}

pub fn is_alphanumeric(c: &char) -> bool {
    ('a'..'z').contains(c) || ('A'..'Z').contains(c) || ('0'..'9').contains(&c)
}
