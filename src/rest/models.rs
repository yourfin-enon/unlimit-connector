use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetQuoteRequest {
    pub amount: String,
    pub crypto: String,
    pub fiat: String,
    #[serde(rename = "partnerAccountId")]
    pub partner_id: String,
    #[serde(rename = "payment")]
    pub payment: String,
    #[serde(rename = "region")]
    pub region: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GateFiPlatformConfigResponse {
    pub version: String,
    #[serde(rename = "updatedAt")]
    pub updated_date: String,
    #[serde(rename = "features")]
    pub features: GateFiPlatformFeatures,
    pub countries: Vec<GateFiPlatformCountry>,
    pub payments: Vec<GateFiPlatformPayment>,
    #[serde(rename = "fiat")]
    pub fiat_assets: Vec<GateFiPlatformAsset>,
    #[serde(rename = "crypto")]
    pub crypto_assets: Vec<GateFiPlatformAsset>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GateFiPlatformAsset {
    pub id: String,
    #[serde(rename = "paymentLimits")]
    pub payment_limits: Option<Vec<GateFiPlatformPaymentLimit>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GateFiPlatformPaymentLimit {
    pub id: String,
    pub min: String,
    pub max: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GateFiPlatformCountry {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GateFiPlatformPayment {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GateFiPlatformFeatures {
    pub quotes: GateFiPlatformFeature,
    pub buy: GateFiPlatformFeature,
    #[serde(rename = "orderTracking")]
    pub order_tracking: GateFiPlatformFeature,
    #[serde(rename = "orderAnalytics")]
    pub order_analytics: GateFiPlatformFeature,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GateFiPlatformFeature {
    #[serde(rename = "enabled")]
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetQuoteResponse {
    #[serde(rename = "processingFee")]
    pub processing_fee: String,
    #[serde(rename = "networkFee")]
    pub network_fee: String,
    #[serde(rename = "amountOut")]
    pub amount_out: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GateFiBuyAssetRequest {
    pub amount: String,
    pub crypto: String,
    pub fiat: String,
    #[serde(rename = "orderCustomId")]
    pub order_custom_id: String,
    #[serde(rename = "partnerAccountId")]
    pub partner_account_id: String,
    #[serde(rename = "payment")]
    pub payment_method: String,
    #[serde(rename = "redirectUrl")]
    pub redirect_url: String,
    pub region: String,
    #[serde(rename = "walletAddress")]
    pub wallet_address: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GateFiBuyAssetResponse {
    pub redirect_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GateFiRatesResponse {
    pub list: HashMap<String, GateFiRates>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GateFiRates {
    pub rates: HashMap<String, f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GateFiPaymentConfigResponse {
    #[serde(rename = "availableNationalities")]
    pub available_nationalities: Vec<String>,
    #[serde(rename = "availableCountries")]
    pub available_countries: Vec<String>,
    #[serde(rename = "fiat")]
    pub fiat_assets: HashMap<String, GateFiGateFiFiatAsset>,
    #[serde(rename = "crypto")]
    pub crypto_assets: HashMap<String, GateFiGateFiCryptoAsset>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GateFiGateFiFiatAsset {
    pub methods: HashMap<String, GateFiGateFiPaymentMethodInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GateFiGateFiPaymentMethodInfo {
    pub min: f64,
    pub max: f64,
    #[serde(rename = "processingFee")]
    pub processing_fee_percent: f64,
    pub precision: i32,
    #[serde(rename = "processingFeeFix")]
    pub processing_fee_fix: f64,
    #[serde(rename = "processingFeeMin")]
    pub processing_fee_min: f64,
    #[serde(rename = "openMode")]
    pub open_mode: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GateFiGateFiCryptoAsset {
    pub title: String,
    #[serde(rename = "type")]
    pub chain: String,
    pub symbol: String,
    #[serde(rename = "chainId")]
    pub chain_id: String,
    #[serde(rename = "networkFee")]
    pub network_fee: f64,
    pub precision: i32,
    pub min: f64,
    pub max: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GateFiGateCallbackRequest {
    pub crypto_amount: String,
    pub crypto_currency: String,
    pub custom_order_id: String,
    pub destination_wallet: String,
    pub fiat_amount: String,
    pub tap_on_fee_amount: Option<String>,
    pub tap_on_fee_currency: Option<String>,
    pub transaction_hashes: Option<Vec<String>>,
    pub transaction_id: String,

}
