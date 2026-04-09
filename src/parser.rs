// Loads aircraft sensor data from a CSV file.

use crate::models::AircraftRecord;
use csv::ReaderBuilder;
use std::error::Error;
use std::fs::File;

/// Loads aircraft records from the given CSV file path.
///
/// Expected headers:
/// aircraft_id,aircraft_model,temperature_c,vibration_mm_s,flight_cycles,oil_pressure_psi
pub fn load_aircraft_data_from_csv(
    file_path: &str,
) -> Result<Vec<AircraftRecord>, Box<dyn Error>> {
    // Open the CSV file.
    let file = File::open(file_path)?;

    // Create a CSV reader that assumes the first row contains headers.
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(file);

    let mut aircraft_data = Vec::new();

    // Deserialize each CSV row into an AircraftRecord.
    for result in reader.deserialize() {
        let record: AircraftRecord = result?;
        aircraft_data.push(record);
    }

    Ok(aircraft_data)
}