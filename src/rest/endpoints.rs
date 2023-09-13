use http::Method;

pub enum GateFiEndpoint {
    PlatformConfig,
    Quotes,
    BuyAsset,
    Rates,
    PaymentConfig,
    PaymentMethods,
}

impl From<&GateFiEndpoint> for String {
    fn from(item: &GateFiEndpoint) -> Self {
        String::from(match item {
            GateFiEndpoint::PlatformConfig => "/onramp/v1/configuration",
            GateFiEndpoint::Quotes => "/onramp/v1/quotes",
            GateFiEndpoint::BuyAsset => "/onramp/v1/buy",
            GateFiEndpoint::Rates => "/api/v1/rates",
            GateFiEndpoint::PaymentConfig => "/api/v1/config",
            GateFiEndpoint::PaymentMethods => "/api/v1/methods/currencies",
        })
    }
}

impl GateFiEndpoint {
    pub fn get_http_method(&self) -> Method {
        match &self {
            GateFiEndpoint::PlatformConfig => Method::GET,
            GateFiEndpoint::Quotes => Method::GET,
            GateFiEndpoint::BuyAsset => Method::GET,
            GateFiEndpoint::Rates => Method::GET,
            GateFiEndpoint::PaymentConfig => Method::GET,
            GateFiEndpoint::PaymentMethods => Method::GET,
        }
    }
}
