//! Holding an API response as given from the Tandoor property-type endpoint.
use serde::{Serialize, Deserialize};
use crate::models::tandoor::api_tandoor_property::ApiTandoorProperty;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiPropertyTypeResponse {
    /// The total number of different properties in the database.
    pub count: i32,
    /// The URL to call to get the next page of properties
    pub next: Option<String>,
    /// List holding [ApiTandoorProperty] representing the requested properties.
    pub results: Vec<ApiTandoorProperty>
}