//! Represents a food item to be sent to the database.
use serde::{Serialize, Deserialize};
use crate::models::tandoor::api_tandoor_food_property::ApiTandoorFoodProperty;
use crate::models::tandoor::internal_tandoor_food::InternalTandoorFood;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiTandoorFood {
    /// The id of the food item
    pub id: i32,
    /// The name of the food item
    pub name: String,
    /// The FDC ID of the food.
    pub fdc_id: Option<i32>,
    /// A list holding all [ApiTandoorFoodProperty] elements of the food item.
    pub properties: Vec<ApiTandoorFoodProperty>,
    /// URL of the food in the FDC database.
    pub url: Option<String>,
}

impl From<InternalTandoorFood> for ApiTandoorFood{
    fn from(value: InternalTandoorFood) -> Self {
        ApiTandoorFood{
            id: value.id,
            name: value.name,
            fdc_id: value.fdc_id,
            properties: value.properties
                .into_iter()
                .map(|x| ApiTandoorFoodProperty::from(x))
                .collect(),
            url: value.url,
        }
    }
}

impl From<&InternalTandoorFood> for ApiTandoorFood{
    fn from(value: &InternalTandoorFood) -> Self {
        ApiTandoorFood{
            id: value.id,
            name: value.name.to_string(),
            fdc_id: value.fdc_id,
            properties: value.properties
                .iter()
                .map(|x| ApiTandoorFoodProperty::from(x.clone()))
                .collect(),
            url: value.url.clone(),
        }
    }
}