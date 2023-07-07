use http::Method;

pub enum GateFiEndpoint {
    GetPlatformConfig,
    GetQuote,
    BuyAsset,
}

impl From<&GateFiEndpoint> for String {
    fn from(item: &GateFiEndpoint) -> Self {
        String::from(match item {
            GateFiEndpoint::GetPlatformConfig => "/onramp/v1/configuration",
            GateFiEndpoint::GetQuote => "/onramp/v1/quotes",
            GateFiEndpoint::BuyAsset => "/onramp/v1/buy",
        })
    }
}

impl GateFiEndpoint {
    pub fn get_http_method(&self) -> Method {
        match &self {
            GateFiEndpoint::GetPlatformConfig => Method::GET,
            GateFiEndpoint::GetQuote => Method::GET,
            GateFiEndpoint::BuyAsset => Method::GET,
        }
    }
}
