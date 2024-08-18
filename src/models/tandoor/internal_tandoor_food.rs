//! Holds a food item as it is returned from the database.
use serde::{Serialize, Deserialize};
use crate::models::tandoor::internal_tandoor_food_property::InternalTandoorFoodProperty;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InternalTandoorFood {
    /// The id of the food item.
    pub id: i32,
    /// The name of the food item.
    pub name: String,
    /// The FDC ID of the food.
    pub fdc_id: Option<i32>,
    /// A list of all [InternalTandoorFoodProperty] that this food has at the moment.
    pub properties: Vec<InternalTandoorFoodProperty>,
    /// URL of the food in the FDC database.
    pub url: Option<String>,
}