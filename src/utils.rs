use colored::*;
use std::io::{self, Write};

pub fn flush_stdout() {
    let _ = io::stdout().flush();
}

pub fn print_prompt() {
    print!("{} ", "user prompt >".blue());
    flush_stdout();
}

pub fn confirm_action(description: &str) -> bool {
    print!("\n是否执行 {} (y/n): ", description);
    flush_stdout();

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return false;
    }
    
    let input = input.trim().to_lowercase();
    input == "y" || input == "yes"
}