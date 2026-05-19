/*
** LUKAS SOIGNEUX PROJECT, 2026
** directory
** File description:
** filename
*/

use tree_sitter::Node;
use crate::coding_style::abstract_syntax_tree::Rules;

const MAX_FUNCTION: usize = 10;
const MAX_NO_STATIC_FUNCTION: usize = 5;

pub fn max_static(rules: &mut Rules) -> Option<String> {
    let mut error: String;

    rules.function += 1;
    if  rules.function - rules.static_function > MAX_NO_STATIC_FUNCTION {
        error = (rules.function - rules.static_function).to_string();
        error.push_str("th non-static");
        return Some(error);
    }
    None
}

pub fn max_function(rules: &mut Rules, node: Node<'_>) -> (Option<String>, usize, usize) {
    let static_error = max_static(rules);
    let mut error: String;
    let line_start = node.start_position().row as usize + 1;
    let line_end = node.end_position().row as usize + 1;

    if rules.function > MAX_FUNCTION {
        error = match static_error.clone() {
            Some(err_msg) => format!("{} and ", err_msg),
            None => String::new(),
        };
        error.push_str(&format!("{}th function in the file", rules.function));
        return (Some(error), line_start, line_end);
    }
    if static_error.is_some() {
        return (static_error, line_start, line_end);
    }
    (None, 0, 0)
}



