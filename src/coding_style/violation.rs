/*
** LUKAS SOIGNEUX PROJECT, 2026
** directory
** File description:
** filename
*/

use std::{fmt, fs};
use colored::Colorize;
use crate::coding_style::error::{self, REFERENCE};

#[derive(Clone, Copy)]
pub enum Level {
    Info,
    Minor,
    Major,
    Fatal
}

pub struct Violation {
    pub reference: &'static str,
    pub line_start: Option<u32>,
    pub line_end: Option<u32>,
    pub column: Option<u32>,
    pub file: Option<String>,
    pub error: Option<String>,
}

impl Violation {
    pub fn new(reference: &'static str, line_start: Option<u32>, line_end: Option<u32>, column: Option<u32>, file: Option<String>, error: Option<String>) -> Self {
        Self { reference, line_start, line_end, column, file, error }
    }

    pub fn get_context(violation: Violation) -> String {
        let file = fs::read_to_string(violation.file.unwrap_or_default())
            .expect(error::ERROR_LOAD_CONTENT);
        let line_start = violation.line_start.unwrap_or(0) as usize;
        let line_end = violation.line_end.unwrap_or(0) as usize;
        
        file.lines()
            .enumerate()
            .skip(line_start)
            .take_while(|(i, _)| *i <= line_end)
            .map(|(_, line)| line.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    }
    
    fn get_level(level: Level) -> &'static str {
        match level {
            Level::Info => error::INFO,
            Level::Minor => error::MINOR,
            Level::Major => error::MAJOR,
            Level::Fatal => error::FATAL
        }
    }

    fn get_error(reference: &'static str) -> (Option<&'static str>, Level) {
        REFERENCE.iter()
            .find(|(code, _, _)| *code == reference)
            .map(|(_, msg, level)| (*msg, *level))
            .unwrap_or((None, Level::Fatal))
    }
}

impl fmt::Display for Violation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (message, level) = Violation::get_error(self.reference);
        
        write!(
            f,
            "{}:{}:{}: {}: {} [{}] {} ({})",
            self.file.as_deref().unwrap_or("?").bold(),
            self.line_start.map_or("?".to_string(), |l| l.to_string()).bold(),
            self.column.map_or("?".to_string(), |c| c.to_string()).bold(),
            "warning".magenta().bold(),
            "[Banana]".bold(),
            Violation::get_level(level).bold(),
            message.unwrap_or_default().bold(),
            self.reference.bold(),
        )
    }
}
