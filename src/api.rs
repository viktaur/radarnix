use crate::models::{Aircraft, Airline, AirportEvent, Coordinates, Flight};
use serde_json::{json, Value};
use reqwest::Result;

pub enum ApiProvider {
    AviationStack
}

pub struct ApiClient {
    provider: ApiProvider,
    key: String,
}

impl ApiClient {
    pub fn new(provider: ApiProvider, key: String) -> Self {
        ApiClient {
            provider,
            key,
        }
    }

    pub async fn get_flight_info(&self, iata_code: &str) -> Result<Flight> {
        match self.provider {
            ApiProvider::AviationStack => {
                let uri = format!(
                    "https://api.aviationstack.com/v1/flights?access_key={}&flight_iata={}",
                    {&self.key},
                    {iata_code}
                );

                self.get_flight_info_aviation_stack(&uri, iata_code).await
            },
        }
    }

    async fn get_flight_info_aviation_stack(
        &self,
        uri: &str,
        iata_code: &str
    ) -> Result<Flight> {
        let json: Value = reqwest::get(uri)
            .await?
            .json()
            .await?;

        let coordinates = Coordinates {
            lat: json["data"][0]["live"]["latitude"].as_f64()
                .expect("Value should be an f64.") as f32,
            lon: json["data"][0]["live"]["longitude"].as_f64()
                .expect("Value should be an f64.") as f32,
        };

        let departure_obj: Value = json["data"][0]["departure"].clone();
        let departure = AirportEvent {
            airport: departure_obj["airport"].as_str().map(|s| s.to_owned()),
            timezone: departure_obj["timezone"].as_str().map(|s| s.parse()
                .expect("Timezone should be valid.")),
            iata: departure_obj["iata"].as_str().map(|s| s.to_owned()),
            icao: departure_obj["icao"].as_str().map(|s| s.to_owned()),
            terminal: departure_obj["timezone"].as_str().map(|s| s.to_owned()),
            gate: departure_obj["gate"].as_str().map(|s| s.to_owned()),
            scheduled_time: departure_obj["scheduled"].as_str().map(|s| s.parse()
                .expect("Scheduled time should be in a valid format")),
            estimated_time: departure_obj["estimated"].as_str().map(|s| s.parse()
                .expect("Estimated time should be in a valid format")),
            actual_time: departure_obj["actual"].as_str().map(|s| s.parse()
                .expect("Actual time should be in a valid format")),
        };

        let arrival_obj: Value = json["data"][0]["arrival"].clone();
        let arrival = AirportEvent {
            airport: arrival_obj["airport"].as_str().map(|s| s.to_owned()),
            timezone: arrival_obj["timezone"].as_str().map(|s| s.parse()
                .expect("Timezone should be valid.")),
            iata: arrival_obj["iata"].as_str().map(|s| s.to_owned()),
            icao: arrival_obj["icao"].as_str().map(|s| s.to_owned()),
            terminal: arrival_obj["timezone"].as_str().map(|s| s.to_owned()),
            gate: arrival_obj["gate"].as_str().map(|s| s.to_owned()),
            scheduled_time: arrival_obj["scheduled"].as_str().map(|s| s.parse()
                .expect("Scheduled time should be in a valid format")),
            estimated_time: arrival_obj["estimated"].as_str().map(|s| s.parse()
                .expect("Estimated time should be in a valid format")),
            actual_time: arrival_obj["actual"].as_str().map(|s| s.parse()
                .expect("Actual time should be in a valid format")),
        };

        let airline_obj = json["data"][0]["airline"].clone();
        let airline = Airline {
            name: airline_obj["name"].as_str().map(|s| s.to_owned()),
            iata: airline_obj["iata"].as_str().map(|s| s.to_owned()),
            icao: airline_obj["iaco"].as_str().map(|s| s.to_owned()),
        };

        let aircraft_obj = json["data"][0]["aircraft"].clone();
        let aircraft = Aircraft {
            registration: aircraft_obj["registration"].as_str().map(|s| s.to_owned()),
            iata: aircraft_obj["iata"].as_str().map(|s| s.to_owned()),
            icao: aircraft_obj["icao"].as_str().map(|s| s.to_owned()),
            icao24: aircraft_obj["icao24"].as_str().map(|s| s.to_owned()),
        };

        let flight = Flight {
            iata_code: iata_code.to_owned(),
            coordinates,
            altitude: json["data"][0]["live"]["altitude"].as_i64()
                .expect("Value should be an i64") as i32,
            departure,
            arrival,
            airline,
            aircraft
        };

        Ok(flight)
    }
}
