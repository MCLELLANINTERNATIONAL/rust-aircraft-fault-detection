// reports.rs
// Generates terminal-based summaries and fault detection reports.

use crate::models::{FaultResult, SensorReading, Severity};

pub fn display_sensor_data(readings: &Vec<SensorReading>) {
    println!("\n=== AIRCRAFT SENSOR DATA ===");
    for reading in readings {
        println!("Aircraft ID      : {}", reading.aircraft_id);
        println!("Aircraft Model   : {}", reading.aircraft_model);
        println!("Component        : {}", reading.component);
        println!("Temperature (°C) : {:.1}", reading.temperature_c);
        println!("Vibration (mm/s) : {:.1}", reading.vibration_mm_s);
        println!("Flight Cycles    : {}", reading.flight_cycles);
        println!("Oil Pressure PSI : {:.1}", reading.oil_pressure_psi);
        println!("----------------------------------------");
    }
}

pub fn detect_faults(readings: &Vec<SensorReading>) -> Vec<FaultResult> {
    let mut results: Vec<FaultResult> = Vec::new();

    for reading in readings {
        let result = reading.evaluate_status();
        results.push(result);
    }

    results
}

pub fn display_fault_report(results: &Vec<FaultResult>) {
    println!("\n=== FAULT DETECTION REPORT ===");

    for result in results {
        println!("Aircraft ID    : {}", result.aircraft_id);
        println!("Aircraft Model : {}", result.aircraft_model);
        println!("Component      : {}", result.component);
        println!("Severity       : {}", result.severity.as_str());

        if result.reasons.is_empty() {
            println!("Reasons        : No abnormal conditions detected.");
        } else {
            println!("Reasons:");
            for reason in &result.reasons {
                println!("  - {}", reason);
            }
        }

        println!("Recommendation : {}", result.recommendation);
        println!(
            "Snapshot       : Temp {:.1}°C | Vib {:.1} mm/s | Cycles {} | Oil {:.1} PSI",
            result.temperature_c,
            result.vibration_mm_s,
            result.flight_cycles,
            result.oil_pressure_psi
        );
        println!("----------------------------------------");
    }
}

pub fn display_summary(results: &Vec<FaultResult>) {
    let mut normal_count = 0;
    let mut warning_count = 0;
    let mut critical_count = 0;

    for result in results {
        match result.severity {
            Severity::Normal => normal_count += 1,
            Severity::Warning => warning_count += 1,
            Severity::Critical => critical_count += 1,
        }
    }

    println!("\n=== SUMMARY ===");
    println!("Normal records   : {}", normal_count);
    println!("Warning records  : {}", warning_count);
    println!("Critical records : {}", critical_count);
    println!("Total records    : {}", results.len());
}