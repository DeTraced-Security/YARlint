use std::path::PathBuf;
use colored::Colorize;

pub fn print_scan_start(path: &str) {
    println!("Scanning {}", path);
}

pub fn print_recursive_warning() {
    println!("{}", format!("Warning: Recursive scanning can consume large amounts of system memory, and may take a long time to complete. Use with caution").yellow());
}

pub fn print_file_summary(count: usize) {
    println!("{}", format!("Found {} YARA files", count).green());
}

pub fn print_valid_file_summary(count: usize) {
    println!("{}", format!("Found {} Valid YARA files", count).green());
}

pub fn print_file(path: &PathBuf) {
    println!("{}", path.display());

}

pub fn print_error(err: &str) {
    eprintln!("{}", format!("Error: {}", err).red())

}