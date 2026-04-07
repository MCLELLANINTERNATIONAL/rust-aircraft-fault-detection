// parser.rs
// Loads sensor data from a CSV file into a Vec<SensorReading>.

use std::fs;
use crate::models::SensorReading;

// Reads the CSV file and returns a vector of sensor readings.
// This uses references and Result for safe error handling.
pub fn load_sensor_data(file_path: &str) -> Result<Vec<SensorReading>, String> {
    let contents = fs::read_to_string(file_path)
        .map_err(|error| format!("Failed to read file '{}': {}", file_path, error))?;

    let mut readings: Vec<SensorReading> = Vec::new();

    for (index, line) in contents.lines().enumerate() {
        // Skip the header row
        if index == 0 {
            continue;
        }

        let columns: Vec<&str> = line.split(',').collect();

        if columns.len() != 7 {
            return Err(format!(
                "Invalid CSV format on line {}. Expected 7 columns, found {}.",
                index + 1,
                columns.len()
            ));
        }

        let reading = SensorReading {
            aircraft_id: columns[0].trim().to_string(),
            aircraft_model: columns[1].trim().to_string(),
            component: columns[2].trim().to_string(),
            temperature_c: columns[3]
                .trim()
                .parse::<f64>()
                .map_err(|_| format!("Invalid temperature on line {}", index + 1))?,
            vibration_mm_s: columns[4]
                .trim()
                .parse::<f64>()
                .map_err(|_| format!("Invalid vibration value on line {}", index + 1))?,
            flight_cycles: columns[5]
                .trim()
                .parse::<u32>()
                .map_err(|_| format!("Invalid flight cycles on line {}", index + 1))?,
            oil_pressure_psi: columns[6]
                .trim()
                .parse::<f64>()
                .map_err(|_| format!("Invalid oil pressure on line {}", index + 1))?,
        };

        readings.push(reading);
    }

    Ok(readings)
}