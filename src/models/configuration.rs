//! Holds the configuration of the program.
use serde::{Serialize, Deserialize};

/// Holds the configuration of the program.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration{
    /// The Tandoor version that is used (either legacy or latest)
    pub tandoor_version: String,
    /// The API key used to access Tandoor resources.
    pub tandoor_api_key: String,
    /// The URL of the Tandoor endpoint.
    pub tandoor_url: String,
    /// The API key used to access USDA FDC resources.
    pub usda_api_key: String
}