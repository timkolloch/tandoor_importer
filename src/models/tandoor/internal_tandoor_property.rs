//! Represents a food property a [InternalTandoorFood] can have.
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InternalTandoorProperty {
    /// The name of the property.
    pub name: String,
    /// The FoodData Central ID of that property.
    pub fdc_id: i32
}