//! Common Functions for GraphQL Benchmarks
//! GraphQL Schema for Benchmarks
pub use async_graphql::{
    Object,
    Context,
    Schema,
    Subscription,
    types::EmptySubscription
};
use serde::{Serialize, Deserialize};

const FLIGHTS: &'static [Flight] = &[
    Flight { id: "test" }
];

#[Object]
impl Flight<'_> {
    async fn id(&self) -> &str {
        &self.id
    }
}

/// Query struct, required for GraphQL
#[derive(Debug, Clone, Copy)]
pub struct Query;

#[Object]
impl Query {
    async fn get_flights(&self, _ctx: &Context<'_>) -> &[Flight] {
        FLIGHTS
    }
}

/// Mutation struct required for GraphQL
#[derive(Debug, Clone, Copy)]
pub struct Mutation;

#[Object]
impl Mutation {
    async fn create_flight(&self) -> bool {
        true
    }
}

#[derive(Serialize, Deserialize)]
struct Flight<'a> {
    id: &'a str
}