// Defines the core data models used by the aircraft fault detection system.

use chrono::Local;
use colored::*;
use serde::Deserialize;

/// Represents one aircraft record loaded from the CSV file.
/// The field names match the CSV column headers exactly so that
/// serde can automatically deserialize each row.
#[derive(Debug, Clone, Deserialize)]
pub struct AircraftRecord {
    pub aircraft_id: String,
    pub aircraft_model: String,
    pub temperature_c: f32,
    pub vibration_mm_s: f32,
    pub flight_cycles: u32,
    pub oil_pressure_psi: u32,
}

impl AircraftRecord {
    /// Displays a readable view of one aircraft's sensor data.
    pub fn display_sensor_data(&self) {
        println!("{}", "----------------------------------------".blue());
        println!("{} {}", "Aircraft ID:".bold(), self.aircraft_id.cyan());
        println!("{} {}", "Model:".bold(), self.aircraft_model.cyan());
        println!("{} {:.1}", "Temperature (C):".bold(), self.temperature_c);
        println!("{} {:.1}", "Vibration (mm/s):".bold(), self.vibration_mm_s);
        println!("{} {}", "Flight Cycles:".bold(), self.flight_cycles);
        println!("{} {}", "Oil Pressure (PSI):".bold(), self.oil_pressure_psi);
        println!("{}", "----------------------------------------".blue());
    }
}

/// Represents the severity level assigned to a detected issue.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Severity {
    Low,
    Moderate,
    High,
    Critical,
}

impl Severity {
    /// Returns a numeric ranking so reports can be sorted from highest to lowest severity.
    pub fn rank(&self) -> u8 {
        match self {
            Severity::Critical => 4,
            Severity::High => 3,
            Severity::Moderate => 2,
            Severity::Low => 1,
        }
    }

    /// Returns the severity as a plain string for logging and display.
    pub fn as_str(&self) -> &str {
        match self {
            Severity::Low => "Low",
            Severity::Moderate => "Moderate",
            Severity::High => "High",
            Severity::Critical => "Critical",
        }
    }

    /// Returns a colorized label for terminal output.
    pub fn colored_label(&self) -> ColoredString {
        match self {
            Severity::Low => "Low".green(),
            Severity::Moderate => "Moderate".yellow(),
            Severity::High => "High".bright_red(),
            Severity::Critical => "Critical".red().bold(),
        }
    }
}

/// Represents the result of fault analysis for a specific aircraft component.
///
/// Multiple results may be created for the same aircraft if the program
/// checks multiple components or systems.
#[derive(Debug, Clone)]
pub struct FaultResult {
    pub timestamp: String,
    pub aircraft_id: String,
    pub aircraft_model: String,
    pub component: String,
    pub severity: Severity,
    pub recommendation: String,
    pub reasons: Vec<String>,
    pub temperature_c: f32,
    pub vibration_mm_s: f32,
    pub flight_cycles: u32,
    pub oil_pressure_psi: u32,
}

impl FaultResult {
    /// Creates a new fault result and stamps it with the current local time.
    pub fn new(
        aircraft_id: &str,
        aircraft_model: &str,
        component: &str,
        severity: Severity,
        recommendation: &str,
        reasons: Vec<String>,
        temperature_c: f32,
        vibration_mm_s: f32,
        flight_cycles: u32,
        oil_pressure_psi: u32,
    ) -> Self {
        Self {
            timestamp: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            aircraft_id: aircraft_id.to_string(),
            aircraft_model: aircraft_model.to_string(),
            component: component.to_string(),
            severity,
            recommendation: recommendation.to_string(),
            reasons,
            temperature_c,
            vibration_mm_s,
            flight_cycles,
            oil_pressure_psi,
        }
    }

    /// Returns true if the result represents a real fault or elevated condition.
    pub fn is_fault(&self) -> bool {
        self.severity != Severity::Low
    }

    /// Displays the fault result in a formatted and colorized way.
    pub fn display(&self) {
        println!("{}", "========================================".blue());
        println!("{} {}", "Scan Time:".bold(), self.timestamp.bright_white());
        println!("{} {}", "Aircraft ID:".bold(), self.aircraft_id.cyan());
        println!("{} {}", "Model:".bold(), self.aircraft_model.cyan());
        println!("{} {}", "Component:".bold(), self.component.magenta());
        println!("{} {}", "Severity:".bold(), self.severity.colored_label());
        println!("{} {}", "Recommendation:".bold(), self.recommendation);

        // If no reasons were added, the component is considered normal.
        if self.reasons.is_empty() {
            println!("{} {}", "Reasons:".bold(), "No abnormal conditions detected".green());
        } else {
            println!("{}", "Reasons:".bold());
            for reason in &self.reasons {
                println!("- {}", reason);
            }
        }

        println!(
            "{} {:.1} | {} {:.1} | {} {} | {} {}",
            "Temp(C):".bold(),
            self.temperature_c,
            "Vibration(mm/s):".bold(),
            self.vibration_mm_s,
            "Cycles:".bold(),
            self.flight_cycles,
            "Oil PSI:".bold(),
            self.oil_pressure_psi
        );
        println!("{}", "========================================".blue());
    }
}