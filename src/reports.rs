// Contains the fault detection logic and report display/filter functions.

use crate::models::{AircraftRecord, FaultResult, Severity};
use colored::*;

/// Generates fault reports for all aircraft in the dataset.
pub fn generate_report_for_all_aircraft(aircraft_data: &[AircraftRecord]) -> Vec<FaultResult> {
    let mut reports = Vec::new();

    // Analyze each aircraft and append all generated component reports.
    for aircraft in aircraft_data {
        let mut aircraft_reports = analyze_aircraft(aircraft);
        reports.append(&mut aircraft_reports);
    }

    // Sort the final report list by severity before returning it.
    sort_reports_by_severity(&mut reports);
    reports
}

/// Generates fault reports for one aircraft by ID.
///
/// Returns None if no aircraft with the requested ID is found.
pub fn generate_report_for_one_aircraft(
    aircraft_id: &str,
    aircraft_data: &[AircraftRecord],
) -> Option<Vec<FaultResult>> {
    for aircraft in aircraft_data {
        if aircraft.aircraft_id == aircraft_id {
            let mut reports = analyze_aircraft(aircraft);
            sort_reports_by_severity(&mut reports);
            return Some(reports);
        }
    }

    None
}

/// Displays all reports generated during the current program session.
pub fn show_summary_all(reports: &[FaultResult]) {
    println!("\n{}", "=== SUMMARY: ALL REPORTS ===".blue().bold());

    if reports.is_empty() {
        println!("{}", "No reports have been generated yet.".yellow());
        return;
    }

    for report in reports {
        report.display();
    }

    println!("{} {}", "Total Reports:".bold(), reports.len());
}

/// Displays only reports for one aircraft.
pub fn show_summary_one(aircraft_id: &str, reports: &[FaultResult]) {
    println!("\n{}", "=== SUMMARY: ONE AIRCRAFT ===".blue().bold());

    // Filter the current session reports to only the requested aircraft.
    let mut filtered: Vec<FaultResult> = reports
        .iter()
        .filter(|r| r.aircraft_id == aircraft_id)
        .cloned()
        .collect();

    if filtered.is_empty() {
        println!("No reports found for aircraft ID '{}'.", aircraft_id);
        return;
    }

    sort_reports_by_severity(&mut filtered);

    for report in &filtered {
        report.display();
    }
}

/// Displays only fault-level reports, excluding low-severity normal-status results.
pub fn show_faults_only(reports: &[FaultResult]) {
    println!("\n{}", "=== SUMMARY: FAULTS ONLY ===".blue().bold());

    // Keep only actual fault results.
    let mut filtered: Vec<FaultResult> =
        reports.iter().filter(|r| r.is_fault()).cloned().collect();

    if filtered.is_empty() {
        println!("{}", "No faulty aircraft reports found.".green());
        return;
    }

    sort_reports_by_severity(&mut filtered);

    for report in &filtered {
        report.display();
    }
}

/// Sorts reports from highest severity to lowest severity.
fn sort_reports_by_severity(reports: &mut [FaultResult]) {
    reports.sort_by(|a, b| b.severity.rank().cmp(&a.severity.rank()));
}

/// Analyzes a single aircraft and returns several component-based fault results.
///
/// This function checks:
/// - engine temperature and oil pressure
/// - vibration / structural concerns
/// - lifecycle / flight cycle wear
fn analyze_aircraft(aircraft: &AircraftRecord) -> Vec<FaultResult> {
    let mut results = Vec::new();

    // ------------------------------
    // ENGINE / THERMAL / OIL ANALYSIS
    // ------------------------------
    let mut engine_reasons = Vec::new();
    let mut engine_severity = Severity::Low;
    let mut engine_recommendation = "Continue monitoring engine conditions".to_string();

    // Check engine temperature thresholds.
    if aircraft.temperature_c > 95.0 {
        engine_reasons.push("Engine temperature is above safe operating threshold".to_string());
        engine_severity = Severity::Critical;
        engine_recommendation =
            "Ground aircraft and inspect engine cooling system immediately".to_string();
    } else if aircraft.temperature_c > 85.0 {
        engine_reasons.push("Engine temperature is elevated".to_string());
        engine_severity = Severity::High;
        engine_recommendation = "Schedule urgent engine inspection".to_string();
    }

    // Check oil pressure thresholds.
    if aircraft.oil_pressure_psi < 18 {
        engine_reasons.push("Oil pressure is dangerously low".to_string());
        engine_severity = Severity::Critical;
        engine_recommendation =
            "Ground aircraft and inspect oil delivery system immediately".to_string();
    } else if aircraft.oil_pressure_psi < 25 && engine_severity.rank() < Severity::High.rank() {
        engine_reasons.push("Oil pressure is below ideal range".to_string());
        engine_severity = Severity::High;
        engine_recommendation =
            "Inspect lubrication system before next flight".to_string();
    }

    // Save the engine-related result.
    results.push(FaultResult::new(
        &aircraft.aircraft_id,
        &aircraft.aircraft_model,
        "Engine System",
        engine_severity,
        &engine_recommendation,
        engine_reasons,
        aircraft.temperature_c,
        aircraft.vibration_mm_s,
        aircraft.flight_cycles,
        aircraft.oil_pressure_psi,
    ));

    // ------------------------------
    // VIBRATION / STRUCTURAL ANALYSIS
    // ------------------------------
    let mut vibration_reasons = Vec::new();
    let mut vibration_severity = Severity::Low;
    let mut vibration_recommendation = "Continue monitoring vibration trends".to_string();

    // Check vibration thresholds.
    if aircraft.vibration_mm_s > 8.0 {
        vibration_reasons.push("Vibration level is critically high".to_string());
        vibration_severity = Severity::Critical;
        vibration_recommendation =
            "Ground aircraft and inspect rotating assemblies immediately".to_string();
    } else if aircraft.vibration_mm_s > 6.0 {
        vibration_reasons.push("Vibration level is above acceptable range".to_string());
        vibration_severity = Severity::High;
        vibration_recommendation =
            "Inspect engine mounts and rotating components soon".to_string();
    } else if aircraft.vibration_mm_s > 4.5 {
        vibration_reasons.push("Vibration level is slightly elevated".to_string());
        vibration_severity = Severity::Moderate;
        vibration_recommendation =
            "Monitor trend and schedule preventive maintenance".to_string();
    }

    // Save the vibration-related result.
    results.push(FaultResult::new(
        &aircraft.aircraft_id,
        &aircraft.aircraft_model,
        "Vibration / Structural System",
        vibration_severity,
        &vibration_recommendation,
        vibration_reasons,
        aircraft.temperature_c,
        aircraft.vibration_mm_s,
        aircraft.flight_cycles,
        aircraft.oil_pressure_psi,
    ));

    // ------------------------------
    // USAGE / LIFECYCLE ANALYSIS
    // ------------------------------
    let mut usage_reasons = Vec::new();
    let mut usage_severity = Severity::Low;
    let mut usage_recommendation =
        "Flight cycle count is within normal monitored range".to_string();

    // Check accumulated flight cycle wear.
    if aircraft.flight_cycles > 12000 {
        usage_reasons.push("Aircraft has very high cumulative flight cycles".to_string());
        usage_severity = Severity::High;
        usage_recommendation =
            "Prioritize heavy maintenance inspection due to accumulated wear".to_string();
    } else if aircraft.flight_cycles > 8000 {
        usage_reasons.push("Aircraft has elevated flight cycles".to_string());
        usage_severity = Severity::Moderate;
        usage_recommendation =
            "Review maintenance intervals and monitor wear-related systems".to_string();
    }

    // Save the usage-related result.
    results.push(FaultResult::new(
        &aircraft.aircraft_id,
        &aircraft.aircraft_model,
        "Usage / Lifecycle",
        usage_severity,
        &usage_recommendation,
        usage_reasons,
        aircraft.temperature_c,
        aircraft.vibration_mm_s,
        aircraft.flight_cycles,
        aircraft.oil_pressure_psi,
    ));

    results
}