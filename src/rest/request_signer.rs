use crate::rest::endpoints::GateFiEndpoint;
use ring::hmac;

#[derive(Debug, Clone)]
pub struct GateFiSigner {}

impl GateFiSigner {
    pub fn generate_sign(key: &str, data: &str) -> String {
        let key = hmac::Key::new(hmac::HMAC_SHA256, key.as_bytes());
        let signature = hmac::sign(&key, data.as_bytes());

        signature
            .as_ref()
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct GateFiRequestSigner {
    secret_key: String,
}

impl GateFiRequestSigner {
    pub fn new(secret_key: String) -> Self {
        Self { secret_key }
    }

    pub fn generate_sign(&self, endpoint: &GateFiEndpoint) -> String {
        let http_method = endpoint.get_http_method();
        let data = format!("{}{}", http_method.as_str(), String::from(endpoint));

        GateFiSigner::generate_sign(&self.secret_key, &data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_sign() {
        let key = "BKxkBRExdqfREsPwEwbydIBSEGssHNAo".to_string();
        let client = GateFiRequestSigner::new(key.clone());

        let sign = client.generate_sign(&GateFiEndpoint::PlatformConfig);

        let source_sign = "e09cb7d69cef805a0f3092c770df60f2e1e91fb3ebdedc8f85f713a7369ba0e5";
        assert_eq!(source_sign, sign);
    }
}
