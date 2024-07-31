//! Represents a property a concrete [InternalTandoorFood] has.
use serde::{Serialize, Deserialize};

use crate::models::tandoor::internal_tandoor_property::InternalTandoorProperty;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InternalTandoorFoodProperty {
    /// How much of the property is in that food.
    pub property_amount: Option<f32>,
    /// Definition of the property that is in that food.
    pub property_type: InternalTandoorProperty
}