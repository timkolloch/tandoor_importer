//! Holds information about a nutrient.
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct USDANutrientInformation{
    /// The ID of the nutrient
    pub id: i32,
    /// The name of the nutrient.
    pub name: String,
    
}