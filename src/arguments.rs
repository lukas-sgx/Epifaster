/*
** EPITECH PROJECT, 2026
** ~/epitech/free/EpiFaster/src
** File description:
** arguments.rs
*/

use std::fs;
use crate::options;

pub struct Parameter {
    pub files: Vec<String>,
    pub supplement_args: Vec<String>
}

fn assign_parameter(files: Vec<String>, supplement_args: Vec<String>) -> Parameter {
    Parameter {
        files: files,
        supplement_args: supplement_args,
    }
}

fn handle_path_argument(arg: &str, files: &mut Vec<String>) -> bool {
    if let Ok(metadata) = fs::metadata(&arg) {
        if metadata.is_file() {
            files.push(arg.to_string());
        }
        return true;
    }
    false
}

pub fn get_parameters(args: Vec<String>) -> Parameter {
    let mut files: Vec<String> = vec![];
    let mut supplement_arg: Vec<String> = vec![];

    for arg in args {
        match arg.as_str() {
            "-h" | "--help" => options::help(),
            "-v" | "--version" => options::version(),
            _ => {
                if !handle_path_argument(&arg, &mut files) {
                    supplement_arg.push(arg.to_string());
                }
            }
        }
    }
    return assign_parameter(files, supplement_arg);
}
