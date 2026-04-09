// Handles user interaction in the terminal.

use crate::models::AircraftRecord;
use colored::*;
use std::io;

/// Prints the main menu.
pub fn print_main_menu() {
    println!("\n{}", "=== AIRCRAFT FAULT DETECTION SYSTEM ===".blue().bold());
    println!("1. View Sensor Data (All Aircraft)");
    println!("2. View Sensor Data (One Aircraft)");
    println!("3. Run Fault Detection (All Aircraft)");
    println!("4. Run Fault Detection (One Aircraft)");
    println!("5. View Summary (All Reports)");
    println!("6. View Summary (One Aircraft)");
    println!("7. View Summary (Faults Only)");
    println!("8. Exit");
    println!("{}", "Enter your choice:".bold());
}

/// Reads the user's menu choice.
pub fn prompt_for_choice() -> String {
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read menu choice");

    choice.trim().to_string()
}

/// Prompts the user to enter an aircraft ID.
pub fn prompt_for_aircraft_id() -> String {
    println!("{}", "Enter aircraft ID:".bold());

    let mut aircraft_id = String::new();
    io::stdin()
        .read_line(&mut aircraft_id)
        .expect("Failed to read aircraft ID");

    aircraft_id.trim().to_string()
}

/// Displays sensor data for all aircraft.
pub fn display_sensor_data_all(aircraft_data: &[AircraftRecord]) {
    println!("\n{}", "=== SENSOR DATA: ALL AIRCRAFT ===".blue().bold());

    for aircraft in aircraft_data {
        aircraft.display_sensor_data();
    }
}

/// Displays sensor data for one aircraft, if found.
pub fn display_sensor_data_one(aircraft_id: &str, aircraft_data: &[AircraftRecord]) {
    let mut found = false;

    for aircraft in aircraft_data {
        if aircraft.aircraft_id == aircraft_id {
            println!("\n{}", "=== SENSOR DATA: ONE AIRCRAFT ===".blue().bold());
            aircraft.display_sensor_data();
            found = true;
            break;
        }
    }

    if !found {
        println!("Aircraft with ID '{}' was not found.", aircraft_id);
    }
}