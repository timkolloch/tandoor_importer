//! Holds a food item as requested from the USDA FDC database.
use serde::{Serialize, Deserialize};

use crate::models::usda::usda_nutrient::USDANutrient;
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct USDAFood{
    /// The FDC ID of the food.
    pub fdc_id: i32,
    /// List of [USDANutrient]s associated with this food.
    pub food_nutrients: Vec<USDANutrient>,
}