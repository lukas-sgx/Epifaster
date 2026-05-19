/*
** LUKAS SOIGNEUX PROJECT, 2026
** directory
** File description:
** filename
*/

pub fn is_snake_case(name: &str) -> bool {
    let mut chars;
    
    if name.is_empty() || name.starts_with('_')
        || name.ends_with('_') || !name.is_ascii() {
        return false;
    }
    chars = name.chars().peekable();
    while let Some(c) = chars.next() {
        if !c.is_ascii_lowercase() && !c.is_ascii_digit() && c != '_' {
            return false;
        }
        if c == '_' && chars.peek() == Some(&'_') {
            return false;
        }
    }
    true
}