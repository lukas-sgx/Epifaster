/*
** LUKAS SOIGNEUX PROJECT, 2026
** directory
** File description:
** filename
*/

use std::fs;
use crate::coding_style::{abstract_syntax_tree::abstract_syntax_tree, error, violation::Violation};

pub fn get_file_content(filename: String) -> String {
    return fs::read_to_string(filename).expect(error::ERROR_LOAD_CONTENT);
}

pub fn checker(file: String, infraction: &mut Violation) {
    abstract_syntax_tree(file, infraction);
}