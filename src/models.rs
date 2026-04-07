// models.rs
// Defines the data structures and methods used by the fault detection system.

#[derive(Debug, Clone)]
pub struct SensorReading {
    pub aircraft_id: String,
    pub aircraft_model: String,
    pub component: String,
    pub temperature_c: f64,
    pub vibration_mm_s: f64,
    pub flight_cycles: u32,
    pub oil_pressure_psi: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Severity {
    Normal,
    Warning,
    Critical,
}

#[derive(Debug, Clone)]
pub struct FaultResult {
    pub aircraft_id: String,
    pub aircraft_model: String,
    pub component: String,
    pub severity: Severity,
    pub recommendation: String,
    pub reasons: Vec<String>,
    pub temperature_c: f64,
    pub vibration_mm_s: f64,
    pub flight_cycles: u32,
    pub oil_pressure_psi: f64,
}

impl Severity {
    pub fn as_str(&self) -> &str {
        match self {
            Severity::Normal => "NORMAL",
            Severity::Warning => "WARNING",
            Severity::Critical => "CRITICAL",
        }
    }
}

impl SensorReading {
    pub fn evaluate_status(&self) -> FaultResult {
        let mut severity = Severity::Normal;
        let mut reasons: Vec<String> = Vec::new();

        if self.temperature_c >= 95.0 {
            severity = Severity::Critical;
            reasons.push(format!(
                "Engine temperature {:.1}°C exceeds critical threshold",
                self.temperature_c
            ));
        } else if self.temperature_c >= 85.0 {
            if severity != Severity::Critical {
                severity = Severity::Warning;
            }
            reasons.push(format!(
                "Engine temperature {:.1}°C exceeds warning threshold",
                self.temperature_c
            ));
        }

        if self.vibration_mm_s >= 5.0 {
            severity = Severity::Critical;
            reasons.push(format!(
                "Vibration {:.1} mm/s exceeds critical threshold",
                self.vibration_mm_s
            ));
        } else if self.vibration_mm_s >= 3.5 {
            if severity != Severity::Critical {
                severity = Severity::Warning;
            }
            reasons.push(format!(
                "Vibration {:.1} mm/s exceeds warning threshold",
                self.vibration_mm_s
            ));
        }

        if self.flight_cycles >= 5000 {
            severity = Severity::Critical;
            reasons.push(format!(
                "Flight cycles {} exceed critical maintenance interval",
                self.flight_cycles
            ));
        } else if self.flight_cycles >= 3500 {
            if severity != Severity::Critical {
                severity = Severity::Warning;
            }
            reasons.push(format!(
                "Flight cycles {} approach maintenance interval",
                self.flight_cycles
            ));
        }

        if self.oil_pressure_psi < 18.0 {
            severity = Severity::Critical;
            reasons.push(format!(
                "Oil pressure {:.1} PSI below critical threshold",
                self.oil_pressure_psi
            ));
        } else if self.oil_pressure_psi < 25.0 {
            if severity != Severity::Critical {
                severity = Severity::Warning;
            }
            reasons.push(format!(
                "Oil pressure {:.1} PSI below normal range",
                self.oil_pressure_psi
            ));
        }

        let recommendation = match severity {
            Severity::Normal => {
                "Continue scheduled monitoring. No immediate action required.".to_string()
            }
            Severity::Warning => {
                "Schedule inspection at next maintenance window and monitor trends.".to_string()
            }
            Severity::Critical => {
                "Immediate maintenance action required before next flight.".to_string()
            }
        };

        FaultResult {
            aircraft_id: self.aircraft_id.clone(),
            aircraft_model: self.aircraft_model.clone(),
            component: self.component.clone(),
            severity,
            recommendation,
            reasons,
            temperature_c: self.temperature_c,
            vibration_mm_s: self.vibration_mm_s,
            flight_cycles: self.flight_cycles,
            oil_pressure_psi: self.oil_pressure_psi,
        }
    }
}