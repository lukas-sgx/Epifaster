/*
** LUKAS SOIGNEUX PROJECT, 2026
** directory
** File description:
** filename
*/

use regex::Regex;
use std::sync::LazyLock;

static EPITECH_HEADER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"/\*\n",
        r"\*\* EPITECH PROJECT, [1-9][0-9]{3}\n",
        r"\*\* \S.+\n",
        r"\*\* File description:\n",
        r"(\*\* .*\n)+",
        r"\*/"
    ))
    .unwrap()
});

pub fn is_valid_epitech_header(header: &String) -> bool {
    EPITECH_HEADER.is_match(header.as_str())
}