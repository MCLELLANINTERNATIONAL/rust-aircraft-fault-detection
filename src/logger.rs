// logger.rs
// Handles advanced persistent fault logging to CSV.

use std::fs::OpenOptions;
use std::io::Write;

use chrono::Local;

use crate::models::FaultResult;

pub fn save_faults(results: &Vec<FaultResult>) -> std::result::Result<(), String> {
    let file_path = "data/fault_log.csv";

    let file_exists = std::path::Path::new(file_path).exists();

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)
        .map_err(|e| format!("Failed to open log file: {}", e))?;

    if !file_exists || std::fs::metadata(file_path).map_err(|e| e.to_string())?.len() == 0 {
        writeln!(
            file,
            "timestamp,aircraft_id,aircraft_model,component,severity,recommendation,reasons,temperature_c,vibration_mm_s,flight_cycles,oil_pressure_psi"
        )
        .map_err(|e| format!("Failed to write header: {}", e))?;
    }

    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    for result in results {
        let reasons_joined = if result.reasons.is_empty() {
            "No abnormal conditions detected".to_string()
        } else {
            result.reasons.join(" | ")
        };

        let safe_recommendation = result.recommendation.replace('"', "\"\"");
        let safe_reasons = reasons_joined.replace('"', "\"\"");

        writeln!(
            file,
            "{},\"{}\",\"{}\",\"{}\",{},\"{}\",\"{}\",{:.1},{:.1},{},{}",
            timestamp,
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