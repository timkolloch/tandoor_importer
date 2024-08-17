//! Represents a food item to be sent to the database.
use serde::{Serialize, Deserialize};
use crate::models::tandoor::api_tandoor_food_property::ApiTandoorFoodProperty;
use crate::models::tandoor::internal_tandoor_food::InternalTandoorFood;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiTandoorFood {
    /// The name of the food item
    pub name: String,
    /// A list holding all [ApiTandoorFoodProperty] elements of the food item.
    pub properties: Vec<ApiTandoorFoodProperty>,
    /// The FDC ID of that food.
    pub fdc_id: Option<i32>
}

impl From<InternalTandoorFood> for ApiTandoorFood{
    fn from(value: InternalTandoorFood) -> Self {
        ApiTandoorFood{
            name: value.name,
            properties: value.properties.into_iter().map(|x| ApiTandoorFoodProperty::from(x)).collect(),
            fdc_id: value.fdc_id
        }
    }
}

impl From<&InternalTandoorFood> for ApiTandoorFood{
    fn from(value: &InternalTandoorFood) -> Self {
        ApiTandoorFood{
            name: value.name.to_string(),
            properties: value.properties.iter().map(|x| ApiTandoorFoodProperty::from(x.clone())).collect(),
            fdc_id: value.fdc_id
        }
    }
}