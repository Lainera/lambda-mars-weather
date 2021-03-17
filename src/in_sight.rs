use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// This module handles InSight request parsing
#[derive(Serialize, Deserialize, Debug)]
pub struct InSight {
    /// List of sols available in response
    sol_keys: Vec<String>,
    /// List of data validity checks
    validity_checks: InSightValidityChecks,
    /// This directive tells SerDe to collect everything
    /// except sol_keys and validity checks into 'sols' HashMap
    /// Serializing InSight with sols populated would flatten it back
    #[serde(flatten)]
    sols: HashMap<String, Value>,
}

impl InSight {
    pub fn earliest_valid_sol(mut self) -> Option<Sol> {
        self.earliest_valid_sol_date()
            .and_then(|date| {
                //remove_entry kindly returns owned (k, v)
                if let Some((date, earliest_valid_sol)) = self.sols.remove_entry(&date) {
                    Some(Sol::from((date, earliest_valid_sol)))
                } else {
                    None
                }
            })
    }

    pub fn valid_sols(&self) -> Vec<&String> {
        self.validity_checks
            .sols
            .iter()
            .filter(|&(_, sol)| sol.is_valid())
            .map(|(key, _)| key)
            .collect()
    }

    pub fn earliest_valid_sol_date(&self) -> Option<String> {
        let mut valid_sols: Vec<i32> = self
            .valid_sols()
            .iter()
            .filter(|&&sol| self.sols.get(sol).is_some())
            .filter_map(|sol| sol.parse::<i32>().ok())
            .collect::<Vec<i32>>();
        //sorts in-place
        valid_sols.sort_unstable();
        valid_sols.first().map(|&x| x.to_string())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sol {
    sol_date: String,
    temperature: SolTemperature,
    wind_direction: HashMap<String, SolWindDirection>,
    pressure: SolPressure,
    horizontal_wind_speed: SolWindSpeed,
    start_utc: DateTime<Utc>,
    end_utc: DateTime<Utc>,
    season: String,
}

impl From<(String, Value)> for Sol {
    fn from((sol_date, mut value): (String, Value)) -> Self {
        let temperature: SolTemperature =
            serde_json::from_value(value["AT"].take()).expect("Failed to parse temperature");
        let wind_direction: HashMap<String, SolWindDirection> =
            serde_json::from_value(value["WD"].take()).expect("Failed to parse wind direction");
        let pressure: SolPressure =
            serde_json::from_value(value["PRE"].take()).expect("Failed to parse pressure");
        let horizontal_wind_speed: SolWindSpeed =
            serde_json::from_value(value["HWS"].take()).expect("Failed to parse wind speed");
        let start_utc: DateTime<Utc> =
            serde_json::from_value(value["First_UTC"].take()).expect("Failed to parse start date");
        let end_utc: DateTime<Utc> =
            serde_json::from_value(value["Last_UTC"].take()).expect("Failed to parse end date");
        let season: String =
            serde_json::from_value(value["Season"].take()).expect("Failed to parse season");
        Sol {
            sol_date,
            temperature,
            wind_direction,
            pressure,
            horizontal_wind_speed,
            start_utc,
            end_utc,
            season,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct SolMeasurements {
    #[serde(rename(deserialize = "av"))]
    average: f64,
    #[serde(rename(deserialize = "mn"))]
    min: f64,
    #[serde(rename(deserialize = "mx"))]
    max: f64,
    #[serde(rename(deserialize = "ct"))]
    sample_count: i32,
}
// exactly the same fields, can type alias
type SolWindSpeed = SolMeasurements;
type SolTemperature = SolMeasurements;
type SolPressure = SolMeasurements;

#[derive(Serialize, Deserialize, Debug)]
struct SolWindDirection {
    compass_degrees: f32,
    compass_point: String,
    compass_right: f64,
    compass_up: f64,
    #[serde(rename(deserialize = "ct"))]
    sample_count: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct InSightValidityChecks {
    sols_checked: Vec<String>,
    sol_hours_required: i32,
    #[serde(flatten)]
    sols: HashMap<String, InSightSolValidation>,
}

#[derive(Serialize, Deserialize, Debug)]
struct InSightSolValidation {
    #[serde(rename(deserialize = "AT"), default = "InSightSolValidationKind::default")]
    temperature: InSightTemperatureValidation,
    #[serde(rename(deserialize = "WD"), default = "InSightSolValidationKind::default")]
    wind_direction: InSightWindDirectionValidation,
    #[serde(rename(deserialize = "PRE"), default = "InSightSolValidationKind::default")]
    pressure: InSightPressureValidation,
    #[serde(rename(deserialize = "HWS"), default = "InSightSolValidationKind::default")]
    horizontal_wind_speed: InSightWindSpeedValidation,
}

impl InSightSolValidation {
    pub fn is_valid(&self) -> bool {
        self.temperature.valid
            && self.wind_direction.valid
            && self.pressure.valid
            && self.horizontal_wind_speed.valid
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct InSightSolValidationKind {
    sol_hours_with_data: Vec<i32>,
    valid: bool,
}

impl Default for InSightSolValidationKind {
    fn default() -> Self {
        InSightSolValidationKind {
            sol_hours_with_data: Vec::new(),
            valid: false,
        }
    }
}
// Same structure, different kinds;
type InSightTemperatureValidation = InSightSolValidationKind;
type InSightWindSpeedValidation = InSightSolValidationKind;
type InSightWindDirectionValidation = InSightSolValidationKind;
type InSightPressureValidation = InSightSolValidationKind;
