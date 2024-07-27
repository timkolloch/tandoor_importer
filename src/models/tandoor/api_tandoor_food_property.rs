//! Represents a single property a [ApiTandoorFood] may have.
use serde::{Serialize, Deserialize};
use crate::models::tandoor::api_tandoor_property::ApiTandoorProperty;
use crate::models::tandoor::internal_tandoor_food_property::InternalTandoorFoodProperty;
use crate::models::usda::usda_nutrient::USDANutrient;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiTandoorFoodProperty {
    /// The amount of the property that is in the food.
    pub property_amount: String,
    /// The property definition.
    pub property_type: ApiTandoorProperty
}

impl From<InternalTandoorFoodProperty> for ApiTandoorFoodProperty{
    fn from(value: InternalTandoorFoodProperty) -> Self {
        ApiTandoorFoodProperty{
            property_amount: value.property_amount.to_string(),
            property_type: ApiTandoorProperty::from(value.property_type),
        }
    }
}

impl From<&USDANutrient> for ApiTandoorFoodProperty{
    fn from(value: &USDANutrient) -> Self {
         ApiTandoorFoodProperty {
            property_amount: value.amount.unwrap_or(0.0).to_string(),
            property_type: ApiTandoorProperty {
                name: value.nutrient_information.name.to_string()
            }
        }
    }
}