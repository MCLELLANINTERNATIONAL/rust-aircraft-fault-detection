// main.rs
// Entry point for the aviation-focused fault detection system.

mod logger;
mod models;
mod parser;
mod reports;
mod ui;

use logger::save_faults;
use parser::load_sensor_data;
use reports::{detect_faults, display_fault_report, display_sensor_data, display_summary};
use ui::{display_menu, get_user_choice};

fn main() {
    let app_name = "Aircraft Fault Detection System";

    println!("Welcome to {}", app_name);
    println!("Loading aviation maintenance sensor data...");

    let readings = match load_sensor_data("data/sample_sensor_data.csv") {
        Ok(data) => data,
        Err(error) => {
            println!("Error: {}", error);
            return;
        }
    };

    let results = detect_faults(&readings);

    loop {
        display_menu();
        let choice = get_user_choice();

        match choice.as_str() {
            "1" => display_sensor_data(&readings),
            "2" => {
                display_fault_report(&results);
                println!("\nSaving advanced fault log...");
                match save_faults(&results) {
                    Ok(_) => println!("Advanced log saved to data/fault_log.csv"),
                    Err(error) => println!("Logging failed: {}", error),
                }
            }
            "3" => display_summary(&results),
            "4" => {
                println!("Exiting {}. Goodbye.", app_name);
                break;
            }
            _ => println!("Invalid choice. Please enter 1, 2, 3, or 4."),
        }
    }
}