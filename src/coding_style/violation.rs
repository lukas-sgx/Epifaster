/*
** LUKAS SOIGNEUX PROJECT, 2026
** directory
** File description:
** filename
*/

use std::fmt;
use colored::Colorize;
use crate::coding_style::{banana, error};

#[derive(Clone, Copy)]
pub enum Level {
    Info,
    Minor,
    Major,
    Fatal
}

pub struct ViolationNode {
    pub reference: &'static str,
    pub line_start: Option<u32>,
    pub line_end: Option<u32>,
    pub column: Option<u32>,
    pub file: Option<String>,
    pub error: Option<String>,
    pub next: Option<Box<ViolationNode>>,
}

pub struct ViolationIter<'actual> {
    next: Option<&'actual ViolationNode>,
}

pub struct Violation {
    head: Option<Box<ViolationNode>>,
}

impl Violation {
    pub fn iter(&self) -> ViolationIter {
        ViolationIter {
            next: self.head.as_deref(),
        }
    }

    pub fn get_warning(&self) {
        let infractions: Vec<&ViolationNode> = self.iter().collect();
    
        for infraction in infractions.into_iter().rev() {
            println!("{infraction}");
            print!("{}", Violation::get_context(infraction));
        }
        println!();
    }
    
    pub fn push(&mut self, reference: &'static str, line_start: Option<u32>, line_end: Option<u32>, column: Option<u32>, file: Option<String>, error: Option<String>) {
        let new_node = Box::new(ViolationNode{
            reference,
            line_start,
            line_end,
            column,
            file,
            error,
            next: self.head.take()
        });

        self.head = Some(new_node);
    }

    pub fn new() -> Self {
        Violation { head: None }
    }

    pub fn get_context(violation: &ViolationNode) -> String {
        let file = banana::get_file_content(violation.file.as_deref().unwrap_or_default().to_string());
        let line_start = violation.line_start.unwrap_or(0) as usize - 1;
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
        error::REFERENCE.iter()
            .find(|(code, _, _)| *code == reference)
            .map(|(_, msg, level)| (*msg, *level))
            .unwrap_or((None, Level::Fatal))
    }
}

impl<'actual> Iterator for ViolationIter<'actual> {
   type Item = &'actual ViolationNode;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            node
        })
    }
}

impl fmt::Display for ViolationNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (message, level) = Violation::get_error(self.reference);
        let error = self.error.clone().unwrap_or_default();
        
        write!(
            f,
            "{}:{}:{}: {}: {} [{}] {} ({})",
            self.file.as_deref().unwrap_or("?").bold(),
            self.line_start.map_or("?".to_string(), |l| l.to_string()).bold(),
            self.column.map_or("?".to_string(), |c| c.to_string()).bold(),
            "warning".magenta().bold(),
            "[Banana]".bold(),
            Violation::get_level(level).bold(),
            message.unwrap_or(error.as_str()).bold(),
            self.reference.bold(),
        )
    }
}
