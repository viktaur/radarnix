use chrono_tz::Tz;
use chrono::{Local, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Flight {
    number: String,
    coordinates: Coordinates,
    altitude: i32,
}


pub struct Coordinates {
    lon: f32,
    lat: f32
}

pub struct Departure {
    airport: String,
    timezone: Tz,
    iata: String,
    icao: String,
    terminal: String,
    gate: String,
    scheduled_time: DateTime<Local>,
    estimated_time: DateTime<Local>,
    actual_time: DateTime<Local>,
}

pub struct Airline {
    name: String,
    iata: String,
    icao: String
}

pub struct Aircraft {
    registration: String,
    iata: String,
    icao: String,
    icao24: String
}
