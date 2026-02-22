//! Represents a food property a food item can have.
use serde::{Serialize, Deserialize};
use crate::models::tandoor::internal_tandoor_property::InternalTandoorProperty;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiTandoorProperty {
    /// The name of the property.
    pub name: String,

    /// The FDC ID of the property.
    pub fdc_id: Option<i32>
}

impl From<InternalTandoorProperty> for ApiTandoorProperty{
    fn from(value: InternalTandoorProperty) -> Self {
        ApiTandoorProperty{
            name: value.name,
            fdc_id: value.fdc_id.into()
        }
    }
}