// ui.rs
// Handles the terminal menu and user input.

use std::io::{self, Write};

pub fn display_menu() {
    println!("\n=== AIRCRAFT FAULT DETECTION SYSTEM ===");
    println!("1. View Sensor Data");
    println!("2. Run Fault Detection");
    println!("3. View Summary");
    println!("4. Exit");
    print!("Enter your choice: ");
    io::stdout().flush().expect("Failed to flush stdout");
}

pub fn get_user_choice() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}