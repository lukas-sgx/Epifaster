/*
** LUKAS SOIGNEUX PROJECT, 2026
** directory
** File description:
** filename
*/

use crate::coding_style::{banana, rules, violation::Violation};
use tree_sitter::{Node, Parser};
use std::path::Path;

pub struct ParsedFile {
    pub tree: tree_sitter::Tree,
}

pub struct Rules {
    pub function: usize,
    pub static_function: usize,
    pub file: String,    
}

pub fn parse(content: &str) -> Option<ParsedFile> {
    let mut parser = Parser::new();
    let lang: tree_sitter::Language = tree_sitter_c::LANGUAGE.into();
    
    parser.set_language(&lang).ok()?;
    let tree = parser.parse(content, None)?;
    Some(ParsedFile { tree })
}

fn handle_function(node: Node<'_>, source_bytes: &[u8], rules: &mut Rules, infraction: &mut Violation) {
    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        if let Ok(text) = child.utf8_text(source_bytes) {
            match child.kind() {
                "storage_class_specifier" if text == "static" => {
                    rules.static_function += 1;
                }
                _ => {}
            }
        }
    }
    if let (Some(name), line, col) = rules::c_o3::max_function(rules, node) {
        Violation::push(infraction, "C-O3", Some(line), Some(col), Some(0), Some(rules.file.clone()), Some(name));
    }
}

fn walk(root_node: Node<'_>, source_bytes: &[u8], filename: String, infraction: &mut Violation) {
    let mut cursor = root_node.walk();
    let mut rules = Rules{function: 0, static_function: 0, file: filename};
    let mut node;

    if let Some(file_trunc) = Path::new(&rules.file).file_stem().and_then(|s| s.to_str()) {
        if !rules::c_o4::is_snake_case(file_trunc) {
            Violation::push(infraction, "C-O4", None, None, None, Some(rules.file.clone()), None);
        }
    }
    if cursor.goto_first_child() {
        loop {
            node = cursor.node();
            match node.kind() {
                "function_definition" => handle_function(node, source_bytes, &mut rules, infraction),
                _ => {}
            }
            if !cursor.goto_next_sibling() {
                break;
            }
        }
        cursor.goto_parent();
    }
}

pub fn abstract_syntax_tree(filename: String, infraction: &mut Violation) {
    let content = banana::get_file_content(filename.clone());
    let parsed = match parse(&content) {
        Some(parse) => parse,
        None => {
            eprintln!("Error parse {}", filename);
            return;
        }
    };
    let root_node = parsed.tree.root_node();

    walk(root_node, content.as_bytes(), filename, infraction);
}