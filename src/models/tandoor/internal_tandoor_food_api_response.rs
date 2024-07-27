//! Holding an API response as given from the Tandoor API when requesting foods.
use serde::{Serialize, Deserialize};
use crate::models::tandoor::internal_tandoor_food::InternalTandoorFood;

#[derive(Debug, Serialize, Deserialize)]
pub struct InternalTandoorFoodApiResponse {
    /// The total number of different foods in the database.
    pub count: i32,
    /// The URL to call to get the next page of foods
    pub next: Option<String>,
    /// List holding [InternalTandoorFood] representing the requested foods.
    pub results: Vec<InternalTandoorFood>
}