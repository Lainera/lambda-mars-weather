use chrono::prelude::*;
use serde::{Deserialize, Serialize};
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
    sols: HashMap<String, InSightSol>,
}

impl InSight {
    pub fn earliest_valid_sol(&self) -> Option<&InSightSol> {
        self.earliest_valid_sol_date()
            .and_then(|date| self.sols.get(&date))
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
pub struct InSightSol {
    /// deserializing Sol would convert raw "AT" key into "temperature"
    /// same could be done by passing in rename(serialzie)
    /// or using both at the same time
    #[serde(rename(deserialize = "AT"))]
    temperature: InSightTemperature,
    #[serde(rename(deserialize = "WD"))]
    wind_direction: HashMap<String, InSightWindDirection>,
    #[serde(rename(deserialize = "PRE"))]
    pressure: InSightPressure,
    #[serde(rename(deserialize = "HWS"))]
    horizontal_wind_speed: InSightWindSpeed,
    /// chrono works with serde out of the box, I just need to specify
    /// correct date format
    #[serde(rename(deserialize = "First_UTC"))]
    start_utc: DateTime<Utc>,
    #[serde(rename(deserialize = "Last_UTC"))]
    end_utc: DateTime<Utc>,
    #[serde(rename(deserialize = "Season"))]
    season: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct InSightMeasurements {
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
type InSightWindSpeed = InSightMeasurements;
type InSightTemperature = InSightMeasurements;
type InSightPressure = InSightMeasurements;
#[derive(Serialize, Deserialize, Debug)]
struct InSightWindDirection {
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
    #[serde(rename(deserialize = "AT"))]
    temperature: InSightTemperatureValidation,
    #[serde(rename(deserialize = "WD"))]
    wind_direction: InSightWindDirectionValidation,
    #[serde(rename(deserialize = "PRE"))]
    pressure: InSightPressureValidation,
    #[serde(rename(deserialize = "HWS"))]
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

// Same structure, different kinds;
type InSightTemperatureValidation = InSightSolValidationKind;
type InSightWindSpeedValidation = InSightSolValidationKind;
type InSightWindDirectionValidation = InSightSolValidationKind;
type InSightPressureValidation = InSightSolValidationKind;
