/*
** LUKAS SOIGNEUX PROJECT, 2026
** directory
** File description:
** filename
*/

use coding_style::violation::Violation;
use std::env;
use colored::Colorize;

mod arguments;
mod build;
mod coding_style;
mod options;

fn show_error(warning_generated: usize, error_compilation: usize) {
    if warning_generated > 0 {
        if warning_generated > 1 {
            eprint!("{}", format!("{warning_generated} warnings").bright_white());
        } else {
            eprint!("{}", format!("{warning_generated} warning").bright_white());
        }
    }
    if error_compilation > 0 {
        if warning_generated > 0 {
            eprint!("{}", " and ".bright_white());
        }
        if error_compilation > 1 {
            eprint!("{}", format!("{error_compilation} errors").bright_white());
        } else {
            eprint!("{}", format!("{error_compilation} error").bright_white());
        }
    }
    if warning_generated > 0 || error_compilation > 0 {
        eprintln!("{}", " generated.".bright_white());
    }
}

pub fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let parameters = arguments::get_parameters(args);
    let files = parameters.files;
    let mut infraction = Violation::new();
    let error_compilation: usize;
    let warning_generated: usize;

    for file in files.clone() {
        coding_style::banana::checker(file.clone(), &mut infraction);
    }
    Violation::get_warning(&infraction);
    error_compilation = build::compiler(files, parameters.supplement_args);
    warning_generated = infraction.iter().count() as usize;
    show_error(warning_generated, error_compilation);
}
