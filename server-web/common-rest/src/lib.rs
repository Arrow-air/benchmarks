//! Common Functions for REST Benchmarks.

use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

/// Represents a flight.
#[derive(Deserialize, Serialize, Debug)]
pub struct Flight<'a> {
    id: &'a str,
    port_depart: &'a str,
    port_arrive: &'a str,
    utc_depart: NaiveDateTime,
    utc_arrive: NaiveDateTime,
    private_charter: bool,
}

/// Represents a flight input.
#[derive(Deserialize, Serialize, Debug)]
pub struct FlightInput {
    /// Departure Vertiport
    pub port_depart: String,
    /// Arrival Vertiport
    pub port_arrive: String,
    /// Time to arrive by (UTC)
    pub utc_arrive_by: NaiveDateTime,
    /// Disallow other parties on the flight
    pub private_charter: bool,
}

/// Returns a list of flights.
pub fn get_flights() -> Vec<Flight<'static>> {
    let flight = Flight {
        id: "EARTH-NAMERICA-FN-0123456789",
        port_depart: "EARTH-NAMERICA-10",
        port_arrive: "EARTH-NAMERICA-11",
        utc_depart: NaiveDate::from_ymd(2022, 8, 18).and_hms(23, 59, 57),
        utc_arrive: NaiveDate::from_ymd(2022, 8, 19).and_hms(0, 22, 10),
        private_charter: false,
    };
    vec![flight]
}
