use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaymentPage {
    host: &'static str,
    config: PaymentPageConfig,
}

impl PaymentPage {
    pub fn new_sandbox(config: PaymentPageConfig) -> Self {
        Self {
            host: "https://onramp-sandbox.gatefi.com",
            config,
        }
    }

    pub fn new_prod(config: PaymentPageConfig) -> Self {
        Self {
            host: "https://onramp.gatefi.com",
            config,
        }
    }

    pub fn into_url(self) -> String {
        let query = serde_qs::to_string(&self.config).expect("must be valid model");

        format!("{}?{}", self.host, query)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaymentPageConfig {
    pub wallet: String,
    #[serde(rename = "walletLock")]
    pub wallet_lock: bool,
    #[serde(rename = "fiatCurrency")]
    pub fiat_currency: String,
    #[serde(rename = "fiatCurrencyLock")]
    pub fiat_currency_lock: bool,
    #[serde(rename = "fiatAmount")]
    pub fiat_amount: String,
    #[serde(rename = "fiatAmountLock")]
    pub fiat_amount_lock: bool,
    #[serde(rename = "cryptoCurrency")]
    pub crypto_currency: String,
    #[serde(rename = "externalId")]
    pub external_id: String,
    #[serde(rename = "partnerAccountId")]
    pub partner_account_id: String,
}
