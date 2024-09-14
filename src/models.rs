use chrono::{DateTime, Local};
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Flight {
    pub iata_code: String,
    pub coordinates: Coordinates,
    pub altitude: i32,
    pub departure: AirportEvent,
    pub arrival: AirportEvent,
    pub airline: Airline,
    pub aircraft: Aircraft
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Coordinates {
    pub lat: f32,
    pub lon: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AirportEvent {
    pub airport: Option<String>,
    pub timezone: Option<Tz>,
    pub iata: Option<String>,
    pub icao: Option<String>,
    pub terminal: Option<String>,
    pub gate: Option<String>,
    pub scheduled_time: Option<DateTime<Local>>,
    pub estimated_time: Option<DateTime<Local>>,
    pub actual_time: Option<DateTime<Local>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Airline {
    pub name: Option<String>,
    pub iata: Option<String>,
    pub icao: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Aircraft {
    pub registration: Option<String>,
    pub iata: Option<String>,
    pub icao: Option<String>,
    pub icao24: Option<String>,
}
