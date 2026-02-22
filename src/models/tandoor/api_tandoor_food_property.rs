//! Represents a single property a [ApiTandoorFood] may have.
use serde::{Serialize, Deserialize};
use crate::models::tandoor::api_tandoor_property::ApiTandoorProperty;
use crate::models::tandoor::internal_tandoor_food_property::InternalTandoorFoodProperty;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiTandoorFoodProperty {
    /// The amount of the property that is in the food.
    pub property_amount: f32,
    /// The property definition.
    pub property_type: ApiTandoorProperty
}

impl From<InternalTandoorFoodProperty> for ApiTandoorFoodProperty{
    fn from(value: InternalTandoorFoodProperty) -> Self {
        ApiTandoorFoodProperty{
            property_amount: value.property_amount.unwrap(),
            property_type: ApiTandoorProperty::from(value.property_type),
        }
    }
}