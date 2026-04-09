// Handles advanced persistent fault logging to CSV.

use std::fs::OpenOptions;
use std::io::Write;

use crate::models::FaultResult;

/// Appends fault results to the CSV log file.
///
/// If the log file is new or empty, a header row is written first.
pub fn save_faults(results: &[FaultResult]) -> Result<(), String> {
    let file_path = "data/fault_log.csv";

    // Check if the file already exists so we know whether to write headers.
    let file_exists = std::fs::metadata(file_path).is_ok();

    // Open the file in append mode or create it if missing.
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)
        .map_err(|e| format!("Failed to open log file: {}", e))?;

    // Write the CSV header if the file is missing or currently empty.
    if !file_exists || std::fs::metadata(file_path).map_err(|e| e.to_string())?.len() == 0 {
        writeln!(
            file,
            "timestamp,aircraft_id,aircraft_model,component,severity,recommendation,reasons,temperature_c,vibration_mm_s,flight_cycles,oil_pressure_psi"
        )
        .map_err(|e| format!("Failed to write header: {}", e))?;
    }

    // Write one row per fault result.
    for result in results {
        // Join reasons into one CSV-safe field.
        let reasons_joined = if result.reasons.is_empty() {
            "No abnormal conditions detected".to_string()
        } else {
            result.reasons.join(" | ")
        };

        // Escape double quotes to preserve CSV formatting.
        let safe_recommendation = result.recommendation.replace('"', "\"\"");
        let safe_reasons = reasons_joined.replace('"', "\"\"");

        writeln!(
            file,
            "{},\"{}\",\"{}\",\"{}\",{},\"{}\",\"{}\",{:.1},{:.1},{},{}",
            result.timestamp,
            result.aircraft_id,
            result.aircraft_model,
            result.component,
            result.severity.as_str(),
            safe_recommendation,
            safe_reasons,
            result.temperature_c,
            result.vibration_mm_s,
            result.flight_cycles,
            result.oil_pressure_psi
        )
        .map_err(|e| format!("Failed to write log row: {}", e))?;
    }

    Ok(())
}