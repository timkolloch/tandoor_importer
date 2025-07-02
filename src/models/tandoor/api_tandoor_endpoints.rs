use std::collections::HashMap;

pub struct ApiEndpoints {
    base_url: String,
    version: String,
    endpoints: HashMap<String, String>,
}

impl ApiEndpoints {
    pub fn new(version: &str, base_url: &str) -> Result<Self, String> {
        Self::with_base_url(version, base_url)
    }

    pub fn with_base_url(version: &str, base_url: &str) -> Result<Self, String> {
        let mut endpoints = HashMap::new();
        let base_url = format!("http://{}/api/", base_url);

        match version {
            "v1" => {
                endpoints.insert("properties".to_string(), format!("{}food-property-type/", base_url));
                endpoints.insert("food".to_string(), format!("{}food/", base_url));
            },
            "v2" => {
                endpoints.insert("properties".to_string(), format!("{}property-type/", base_url));
                endpoints.insert("food".to_string(), format!("{}food/", base_url));
            },
            _ => return Err(format!("The given API version is not supported: {}", version))
        }

        Ok(ApiEndpoints {
            base_url,
            version: version.to_string(),
            endpoints,
        })
    }

    pub fn get_endpoint_properties(&self) -> &String {
        self.endpoints.get("properties").expect("Missing endpoint for food property retrieval.")
    }

    pub fn get_endpoint_food(&self) -> &String {
        self.endpoints.get("food").expect("Missing endpoint for food retrieval")
    }

    pub fn get_endpoint(&self, name: &str) -> Option<&String> {
        self.endpoints.get(name)
    }

    pub fn get_all_endpoints(&self) -> &HashMap<String, String> {
        &self.endpoints
    }

    pub fn get_version(&self) -> &str {
        &self.version
    }

    pub fn get_base_url(&self) -> &str {
        &self.base_url
    }
}