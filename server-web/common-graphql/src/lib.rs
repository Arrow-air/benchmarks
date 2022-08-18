//! Common Functions for GraphQL Benchmarks
//! GraphQL Schema for Benchmarks
pub use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    types::EmptySubscription,
    Context, InputObject, Object, Schema, SimpleObject, Subscription,
};
use chrono::{NaiveDate, NaiveDateTime};

/// GraphQL Schema for Flight Plans
pub type FlightSchema = Schema<Query, Mutation, EmptySubscription>;

#[derive(InputObject)]
struct FlightInput {
    port_depart: String,
    port_arrive: String,
    utc_arrive_by: NaiveDateTime,
    private_charter: bool,
}

/// Flight Plan Information
#[derive(Debug)]
pub struct Flight<'a> {
    id: &'a str,
    port_depart: &'a str,
    port_arrive: &'a str,
    utc_depart: NaiveDateTime,
    utc_arrive: NaiveDateTime,
    private_charter: bool,
}

#[Object]
impl Flight<'_> {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn port_depart(&self) -> &str {
        &self.port_depart
    }

    async fn port_arrive(&self) -> &str {
        &self.port_arrive
    }

    async fn utc_depart(&self) -> NaiveDateTime {
        self.utc_depart
    }

    async fn utc_arrive(&self) -> NaiveDateTime {
        self.utc_arrive
    }

    async fn private_charter(&self) -> bool {
        self.private_charter
    }
}

/// Query struct, required for GraphQL
#[derive(Debug, Clone, Copy)]
pub struct Query;

#[Object]
impl Query {
    async fn get_flights(&self, _ctx: &Context<'_>) -> Vec<Flight> {
        vec![Flight {
            id: "EARTH-NAMERICA-FN-0123456789",
            port_depart: "EARTH-NAMERICA-10",
            port_arrive: "EARTH-NAMERICA-11",
            utc_depart: NaiveDate::from_ymd(2022, 8, 18).and_hms(23, 59, 57),
            utc_arrive: NaiveDate::from_ymd(2022, 8, 19).and_hms(0, 22, 10),
            private_charter: false,
        }]
    }
}

/// Mutation struct required for GraphQL
#[derive(Debug, Clone, Copy)]
pub struct Mutation;

#[Object]
impl Mutation {
    async fn request_flight(&self, _flight: FlightInput) -> bool {
        true
    }
}
