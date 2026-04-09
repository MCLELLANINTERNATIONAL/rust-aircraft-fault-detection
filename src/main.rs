// Main program file for the Aircraft Fault Detection System.

mod logger;
mod models;
mod parser;
mod reports;
mod ui;

use logger::save_faults;
use parser::load_aircraft_data_from_csv;
use reports::{
    generate_report_for_all_aircraft,
    generate_report_for_one_aircraft,
    show_faults_only,
    show_summary_all,
    show_summary_one,
};
use ui::{
    display_sensor_data_all,
    display_sensor_data_one,
    print_main_menu,
    prompt_for_aircraft_id,
    prompt_for_choice,
};

fn main() {
    // Path to the aircraft sensor data file.
    let data_file = "data/sample_sensor_data.csv";

    // Attempt to load the CSV data before starting the menu loop.
    let aircraft_data = match load_aircraft_data_from_csv(data_file) {
        Ok(data) => data,
        Err(error) => {
            eprintln!("Failed to load aircraft data: {}", error);
            return;
        }
    };

    // Stores all reports generated during the current run of the program.
    let mut session_reports = Vec::new();

    // Main application loop.
    loop {
        print_main_menu();
        let choice = prompt_for_choice();

        match choice.trim() {
            // View sensor data for all aircraft.
            "1" => {
                display_sensor_data_all(&aircraft_data);
            }

            // View sensor data for one selected aircraft.
            "2" => {
                let aircraft_id = prompt_for_aircraft_id();
                display_sensor_data_one(&aircraft_id, &aircraft_data);
            }

            // Run fault detection for all aircraft and save results.
            "3" => {
                let reports = generate_report_for_all_aircraft(&aircraft_data);

                println!("\n--- Fault Detection Report: All Aircraft ---");
                for report in &reports {
                    report.display();
                }

                if let Err(error) = save_faults(&reports) {
                    eprintln!("Failed to save fault log: {}", error);
                }

                // Extend the session report list with all generated reports.
                session_reports.extend(reports);
            }

            // Run fault detection for one aircraft and save results.
            "4" => {
                let aircraft_id = prompt_for_aircraft_id();

                match generate_report_for_one_aircraft(&aircraft_id, &aircraft_data) {
                    Some(reports) => {
                        println!("\n--- Fault Detection Report: One Aircraft ---");
                        for report in &reports {
                            report.display();
                        }

                        if let Err(error) = save_faults(&reports) {
                            eprintln!("Failed to save fault log: {}", error);
                        }

                        session_reports.extend(reports);
                    }
                    None => {
                        println!("Aircraft with ID '{}' was not found.", aircraft_id);
                    }
                }
            }

            // View summary for all reports generated this session.
            "5" => {
                show_summary_all(&session_reports);
            }

            // View summary for one selected aircraft.
            "6" => {
                let aircraft_id = prompt_for_aircraft_id();
                show_summary_one(&aircraft_id, &session_reports);
            }

            // View only reports that represent actual faults.
            "7" => {
                show_faults_only(&session_reports);
            }

            // Exit the program.
            "8" => {
                println!("Exiting Aircraft Fault Detection System.");
                break;
            }

            // Handle invalid menu input.
            _ => {
                println!("Invalid option. Please enter a number from 1 to 8.");
            }
        }
    }
}