//! Holds a food item as it is returned from the database.
use serde::{Serialize, Deserialize};
use crate::models::tandoor::api_tandoor_food::ApiTandoorFood;
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

impl TryFrom<ApiTandoorFood> for InternalTandoorFood {
    type Error = ();
    fn try_from(value: ApiTandoorFood) -> Result<Self, Self::Error> {
        Ok(Self{
            id: value.id,
            name: value.name,
            fdc_id: value.fdc_id,
            properties: value.properties
                .into_iter()
                .map(|api_food_property| InternalTandoorFoodProperty::try_from(api_food_property))
                .filter_map(Result::ok)
                .collect(),
            url: value.url,
        })
    }
}