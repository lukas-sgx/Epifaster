/*
** EPITECH PROJECT, 2026
** ~/epitech/free/EpiFaster/src
** File description:
** build.rs
*/

use std::process::Command;

const EPICLANG_COMPILER: &str = "clang";

pub fn compiler(files: Vec<String>, supplement_args: Vec<String>) -> usize {
    let output = Command::new(EPICLANG_COMPILER)
        .arg("-fdiagnostics-color=always")
        .args(files)
        .args(supplement_args)
        .output()
        .expect("Epiclang fail to build");
    let stderr_output = String::from_utf8_lossy(&output.stderr);
    let mut error_n = 0;
    let mut clean_str: String = String::new();
    
    for line in stderr_output.lines() {
        if line.contains("generated.") && (line.contains("error") || line.contains("errors")) {
            if let Some(count) = line.split_whitespace().next().and_then(|str| {
                clean_str = str.chars().filter(|char| char.is_ascii_digit()).collect();
                clean_str.parse::<usize>().ok()
            }) {
                error_n = count;
            }
            break;
        }
        eprintln!("{line}");
    }
    error_n
}
