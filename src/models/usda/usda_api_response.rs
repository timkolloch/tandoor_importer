//! Holds the answer to a food request to the USDA FDC database.
use serde::{Serialize, Deserialize};
use crate::models::usda::usda_food::USDAFood;
#[derive(Debug, Serialize, Deserialize)]
pub struct USDAApiResponse{
    /// The number of requests left before the IP gets rate-limited.
    pub requests_left: i32,
    /// The representation of the requested [USDAFood].
    pub food: USDAFood,
}