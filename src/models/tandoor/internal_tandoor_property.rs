//! Represents a food property a [InternalTandoorFood] can have.
use serde::{Serialize, Deserialize};
use crate::models::tandoor::api_tandoor_property::ApiTandoorProperty;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InternalTandoorProperty {
    /// The name of the property.
    pub name: String,
    /// The FoodData Central ID of that property.
    pub fdc_id: Option<i32>
}

impl TryFrom<ApiTandoorProperty> for InternalTandoorProperty {
    type Error = ();
    fn try_from(value: ApiTandoorProperty) -> Result<Self, Self::Error> {        
        Ok(Self{
            name: value.name,
            fdc_id: value.fdc_id
        })
    }
}