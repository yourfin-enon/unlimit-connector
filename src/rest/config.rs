#[derive(Clone, Debug)]
pub struct GateFiApiConfig {
    pub rest_api_host: String,
}

impl GateFiApiConfig {
    pub fn sandbox() -> Self {
        Self {
            rest_api_host: "https://api-sandbox.gatefi.com".into(),
        }
    }

    pub fn prod() -> Self {
        Self {
            rest_api_host: "https://api.gatefi.com".into(),
        }
    }
}
