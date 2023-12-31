use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

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
pub struct GateFiPaymentMethodsRequest {
    #[serde(rename = "currencyIso")]
    pub currency_iso: String,
    #[serde(rename = "countryCode")]
    pub country_code: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GateFiPaymentMethodsResponse {
    #[serde(rename = "list")]
    pub list: Option<Vec<GateFiCurrencyPaymentMethod>>,
    #[serde(rename = "total")]
    pub total: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GateFiCurrencyPaymentMethod {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "processingFee")]
    pub processing_fee_percent: f64,
    #[serde(rename = "processingFeeFix")]
    pub processing_fee_fix: f64,
    #[serde(rename = "processingFeeMin")]
    pub processing_fee_min: f64,
    #[serde(rename = "openMode")]
    pub open_mode: String,
    #[serde(rename = "title")]
    pub title: String,
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
    pub fiat_assets: HashMap<String, GateFiFiatAsset>,
    #[serde(rename = "crypto")]
    pub crypto_assets: HashMap<String, GateFiCryptoAsset>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GateFiFiatAsset {
    pub methods: HashMap<String, GateFiPaymentMethodInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GateFiPaymentMethodInfo {
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
pub struct GateFiCryptoAsset {
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
pub struct GateFiCallbackData {
    #[serde(rename = "cryptoAmount")]
    pub crypto_amount: String,
    #[serde(rename = "cryptoCurrency")]
    pub crypto_currency: String,
    #[serde(rename = "customOrderId")]
    pub custom_order_id: String,
    #[serde(rename = "destinationWallet")]
    pub destination_wallet: String,
    #[serde(rename = "fiatCurrency")]
    pub fiat_currency: String,
    #[serde(rename = "fiatAmount")]
    pub fiat_amount: String,
    #[serde(rename = "tapOnFeeAmount")]
    pub tap_on_fee_amount: Option<String>,
    #[serde(rename = "tapOnFeeCurrency")]
    pub tap_on_fee_currency: Option<String>,
    #[serde(rename = "transactionHashes")]
    pub transaction_hashes: Option<Vec<String>>,
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    pub status: String,
    #[serde(rename = "failReason")]
    pub fail_reason: Option<String>,
    #[serde(rename = "paymentMethod")]
    pub payment_method: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum GateFiTransactionStatus {
    Init = 0,
    Created = 1,
    Succeeded = 2,
    Failed = 3,
}

impl fmt::Display for GateFiTransactionStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GateFiTransactionStatus::Init => write!(f, "init"),
            GateFiTransactionStatus::Created => write!(f, "created"),
            GateFiTransactionStatus::Succeeded => write!(f, "succeeded"),
            GateFiTransactionStatus::Failed => write!(f, "failed"),
        }
    }
}
