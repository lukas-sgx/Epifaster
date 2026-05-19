/*
** LUKAS SOIGNEUX PROJECT, 2026
** directory
** File description:
** filename
*/

use crate::coding_style::violation::Level;

pub const ERROR_LOAD_CONTENT: &str = "Error load file content";


pub const INFO: &str = "Info";
pub const MINOR: &str = "Minor";
pub const MAJOR: &str = "Major";
pub const FATAL: &str = "Fatal";


pub const C_A1: &str = "pointer must be a constant when not modified in the function";
pub const C_A3: &str = "file not ending with a newline";
pub const C_F6: &str = "function pointer without parameter";
pub const C_F7: &str = "structure received by copy";
pub const C_F8: &str = "comment inside function";
pub const C_G1: &str = "file not starting with standard Epitech header";
pub const C_G4: &str = "global variable";
pub const C_G5: &str = "non-header file include";
pub const C_G7: &str = "trailing space";
pub const C_G8: &str = "trailing empty line";
pub const C_G10: &str = "inline assembler is forbidden";
pub const C_O1: &str = "temp files are forbidden";
pub const C_O2: &str = "source file must be .c or .h";
pub const C_O4: &str = "file name must be according to the snake_case convention";


pub const REFERENCE: &[(&'static str, Option<&str>, Level)] = &[
    ("C-A1", Some(C_A1), Level::Info),
    ("C-A3", Some(C_A3), Level::Info),
    ("C-C1", None, Level::Major),
    ("C-C2", None, Level::Major),
    ("C-C3", None, Level::Major),
    ("C-F2", None, Level::Minor),
    ("C-F3", None, Level::Major),
    ("C-F4", None, Level::Major),
    ("C-F5", None, Level::Major),
    ("C-F6", Some(C_F6), Level::Major),
    ("C-F7", Some(C_F7), Level::Major),
    ("C-F8", Some(C_F8), Level::Minor),
    ("C-G1", Some(C_G1), Level::Minor),
    ("C-G2", None, Level::Minor),
    ("C-G3", None, Level::Minor),
    ("C-G4", Some(C_G4), Level::Major),
    ("C-G5", Some(C_G5), Level::Major),
    ("C-G6", None, Level::Minor),
    ("C-G7", Some(C_G7), Level::Minor),
    ("C-G8", Some(C_G8), Level::Minor),
    ("C-G10", Some(C_G10), Level::Fatal),
    ("C-H1", None, Level::Major),
    ("C-H2", None, Level::Major),
    ("C-H3", None, Level::Major),
    ("C-L1", None, Level::Major),
    ("C-L2", None, Level::Minor),
    ("C-L3", None, Level::Minor),
    ("C-L4", None, Level::Minor),
    ("C-L5", None, Level::Major),
    ("C-L6", None, Level::Minor),
    ("C-O1", Some(C_O1), Level::Major),
    ("C-O2", Some(C_O2), Level::Major),
    ("C-O3", None, Level::Major),
    ("C-O4", Some(C_O4), Level::Minor),
    ("C-V1", None, Level::Minor),
    ("C-V3", None, Level::Minor),
];