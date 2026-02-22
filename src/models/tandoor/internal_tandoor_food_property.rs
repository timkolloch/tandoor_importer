//! Represents a property a concrete [InternalTandoorFood] has.
use serde::{Serialize, Deserialize};
use crate::models::tandoor::api_tandoor_food_property::ApiTandoorFoodProperty;
use crate::models::tandoor::internal_tandoor_property::InternalTandoorProperty;
use crate::models::usda::usda_nutrient::USDANutrient;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InternalTandoorFoodProperty {
    /// How much of the property is in that food.
    pub property_amount: Option<f32>,
    /// Definition of the property that is in that food.
    pub property_type: InternalTandoorProperty
}

impl From<&USDANutrient> for InternalTandoorFoodProperty{
    fn from(value: &USDANutrient) -> Self {
        InternalTandoorFoodProperty {
            property_amount: value.amount,
            property_type: InternalTandoorProperty {
                name: value.nutrient_information.name.to_string(),
                fdc_id: Option::from(value.nutrient_information.id)
            }
        }
    }
}

impl TryFrom<ApiTandoorFoodProperty> for InternalTandoorFoodProperty {
    type Error = ();
    
    fn try_from(value: ApiTandoorFoodProperty) -> Result<Self, Self::Error> {
        Ok(Self{
            property_amount: Option::from(value.property_amount),
            property_type: InternalTandoorProperty::try_from(value.property_type)?,
        })
    }
}