/*
** EPITECH PROJECT, 2026
** ~/epitech/free/EpiFaster/src
** File description:
** option.rs
*/

use std::process;

const VERSION: &str = "0.0.1";

pub fn version()
{
    println!("epiclang v{VERSION}");
    process::exit(0);
}

pub fn help() {
    println!("epiclang v{VERSION}");
    println!();
    println!("USAGE:");
    println!("    epiclang <file.c>...");
    println!();
    println!("OPTIONS:");
    println!("    -h, --help\t\tPrint help information");
    println!("    -v, --version\tPrint version information");
    process::exit(0);
}
