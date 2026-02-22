//! Holding an API response as given from the Tandoor API when requesting foods.
use serde::{Serialize, Deserialize};
use crate::models::tandoor::api_tandoor_food::ApiTandoorFood;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiFoodResponse {
    /// The total number of different foods in the database.
    pub count: i32,
    /// The URL to call to get the next page of foods
    pub next: Option<String>,
    /// List holding [ApiTandoorFood] representing the requested foods.
    pub results: Vec<ApiTandoorFood>,
}