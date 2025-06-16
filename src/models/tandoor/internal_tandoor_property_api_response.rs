//! Holding an API response as given from the Tandoor API when requesting properties.
use serde::{Serialize, Deserialize};
use crate::models::tandoor::internal_tandoor_property::InternalTandoorProperty;

#[derive(Debug, Serialize, Deserialize)]
pub struct InternalTandoorPropertyApiResponse {
    /// The total number of different properties in the database.
    pub count: i32,
    /// The URL to call to get the next page of properties
    pub next: Option<String>,
    /// List holding [InternalTandoorProperty] representing the requested properties.
    pub results: Vec<InternalTandoorProperty>
}