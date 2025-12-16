use colored::*;
use std::io::{self, Write};

const DANGEROUS_CMDS: [&str; 14] = [
    "rm ",
    "rmdir ",
    "rm -rf",
    "rm -r ",
    "del ",
    "format ",
    "shutdown",
    "sudo ",
    "su ",
    "chmod 000",
    "> /dev",
    "dd if=",
    "killall",
    "mkfs",
];

const SENSITIVE_PATHS: [&str; 15] = [
    "/etc/",
    "/var/",
    "/usr/",
    "/bin/",
    "/sbin/",
    "/root/",
    "C:\\Windows\\",
    ".ssh/",
    "/proc/",
    "/sys/",
    "/boot/",
    "/dev/",
    "/lib/",
    "/usr/local/",
    "/usr/share/",
];

pub fn flush_stdout() {
    let _ = io::stdout().flush();
}

pub fn print_prompt() {
    print!("{} ", "user prompt >".blue());
    flush_stdout();
}

fn get_input() -> String {
    let mut input = String::new();
    if let Err(_) = io::stdin().read_line(&mut input) {
        return String::new();
    }
    input.trim().to_lowercase()
}

pub fn confirm_tool_call(tool_name: &str, params: &str) -> bool {
    print!("\n调用 {} ({}), 执行? (y/n): ", tool_name, params);
    flush_stdout();

    get_input() == "y" || get_input() == "yes"
}

pub fn confirm_dangerous(description: &str) -> bool {
    print!("\n危险操作: {}, 确认? (yes/no): ", description);
    flush_stdout();

    get_input() == "yes"
}

pub fn is_dangerous_command(cmd: &str) -> bool {
    let cmd_lower = cmd.to_lowercase();
    DANGEROUS_CMDS
        .iter()
        .any(|&keyword| cmd_lower.contains(keyword))
}

pub fn is_sensitive_path(path: &str) -> bool {
    let path_normalized = path.replace('\\', "/");
    SENSITIVE_PATHS
        .iter()
        .any(|&pattern| path_normalized.contains(pattern))
}
