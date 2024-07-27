//! Holds the representation of a single nutrient of a food item.
use serde::{Serialize, Deserialize};
use crate::models::usda::usda_nutrient_information::USDANutrientInformation;

#[derive(Debug, Serialize, Deserialize)]
pub struct USDANutrient{
    /// How much of that nutrient is in the food.
    pub amount: Option<f32>,

    /// Contains the description of the nutrient.
    #[serde(alias = "nutrient")]
    pub nutrient_information: USDANutrientInformation
}