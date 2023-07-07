use crate::rest::config::GateFiApiConfig;
use crate::rest::endpoints::GateFiEndpoint;
use crate::rest::errors::Error;
use crate::rest::models::{
    GateFiBuyAssetRequest, GateFiBuyAssetResponse, GateFiPlatformConfigResponse, GetQuoteRequest,
    GetQuoteResponse,
};
use crate::rest::request_signer::RequestSigner;
use error_chain::bail;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Response;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::fmt;

#[derive(Clone)]
pub struct GateFiRestClient {
    signer: RequestSigner,
    access_key: String,
    host: String,
    inner_client: reqwest::Client,
    partner_id: String,
}

impl GateFiRestClient {
    pub fn new(
        partner_id: String,
        secret_key: String,
        access_key: String,
        config: GateFiApiConfig,
    ) -> Self {
        Self {
            signer: RequestSigner::new(secret_key),
            access_key,
            host: config.rest_api_host,
            inner_client: reqwest::Client::new(),
            partner_id,
        }
    }

    pub async fn get_quote(
        &self,
        amount: impl Into<String>,
        crypto_asset: impl Into<String>,
        fiat_asset: impl Into<String>,
        payment_method: &GateFiBuyAssetPaymentMethod,
        region: impl Into<String>,
    ) -> Result<GetQuoteResponse, Error> {
        let request = GetQuoteRequest {
            amount: amount.into(),
            crypto: crypto_asset.into(),
            fiat: fiat_asset.into(),
            partner_id: self.partner_id.clone(),
            payment: payment_method.to_string(),
            region: region.into(),
        };
        let query_string = serde_qs::to_string(&request).unwrap(); // todo: handle err
        let resp: GetQuoteResponse = self
            .get_signed(GateFiEndpoint::GetQuote, Some(&query_string))
            .await?;

        Ok(resp)
    }

    pub async fn buy_asset(
        &self,
        params: GateFiBuyAssetParams,
    ) -> Result<GateFiBuyAssetResponse, Error> {
        let request = GateFiBuyAssetRequest {
            amount: params.amount,
            crypto: params.crypto,
            fiat: params.fiat,
            order_custom_id: params.order_custom_id,
            partner_account_id: self.partner_id.clone(),
            payment_method: params.payment_method.to_string(),
            redirect_url: params.redirect_url,
            region: params.region,
            wallet_address: params.wallet_address,
        };
        let query_params = serde_qs::to_string(&request).unwrap(); // todo: handle err
        let endpoint = GateFiEndpoint::BuyAsset;
        let url = format!("{}{}?{}", self.host, String::from(&endpoint), query_params);
        let sign = self.signer.generate_sign(&endpoint);
        let client = &self.inner_client;
        let headers = self.build_headers(Some(&sign));
        let response = client.get(url.as_str()).headers(headers).send().await?;

        if response.status() != StatusCode::OK {
            return self.handler(response, None).await;
        }

        Ok(GateFiBuyAssetResponse {
            redirect_url: response.url().to_string(),
        })
    }

    pub async fn get_platform_config(&self) -> Result<GateFiPlatformConfigResponse, Error> {
        let resp: GateFiPlatformConfigResponse = self
            .get_signed(GateFiEndpoint::GetPlatformConfig, None)
            .await?;

        Ok(resp)
    }

    pub async fn post_signed<T: DeserializeOwned>(
        &self,
        endpoint: GateFiEndpoint,
        request_json: String,
    ) -> Result<T, Error> {
        let url: String = format!("{}{}", self.host, String::from(&endpoint));
        let sign = self.signer.generate_sign(&endpoint);

        let headers = self.build_headers(Some(&sign));
        let client = &self.inner_client;
        let response = client
            .post(&url)
            .body(request_json.clone())
            .headers(headers)
            .send()
            .await?;

        self.handler(response, Some(request_json)).await
    }

    pub async fn get_signed<T: DeserializeOwned>(
        &self,
        endpoint: GateFiEndpoint,
        query_params: Option<&str>,
    ) -> Result<T, Error> {
        let url: String = if let Some(query_params) = query_params {
            format!("{}{}?{}", self.host, String::from(&endpoint), query_params)
        } else {
            format!("{}{}", self.host, String::from(&endpoint))
        };
        let sign = self.signer.generate_sign(&endpoint);

        let client = &self.inner_client;
        let headers = self.build_headers(Some(&sign));
        let response = client.get(url.as_str()).headers(headers).send().await?;

        self.handler(response, None).await
    }

    fn build_headers(&self, sign: Option<&str>) -> HeaderMap {
        let mut custom_headers = HeaderMap::new();

        custom_headers.insert(
            "access-control-allow-headers",
            HeaderValue::from_str("Accept").unwrap(),
        );

        custom_headers.insert(
            HeaderName::from_static("api-key"),
            HeaderValue::from_str(&self.access_key).unwrap(),
        );

        if let Some(sign) = sign {
            custom_headers.insert(
                HeaderName::from_static("signature"),
                HeaderValue::from_str(sign).unwrap(),
            );
        }

        custom_headers
    }

    pub fn build_query(&self, parameters: HashMap<String, String>) -> String {
        let mut request = String::new();
        for (key, value) in parameters {
            let param = format!("{key}={value}&");
            request.push_str(param.as_ref());
        }
        request.pop();
        request
    }

    async fn handler<T: DeserializeOwned>(
        &self,
        response: Response,
        request_json: Option<String>,
    ) -> Result<T, Error> {
        match response.status() {
            StatusCode::OK => {
                let json: Result<String, _> = response.text().await;
                let Ok(json) = json else {
                    bail!("Failed to read response body");
                };

                let body: Result<T, _> = serde_json::from_str(&json);
                if let Err(err) = body {
                    bail!("Failed to deserialize body {:?}: {}", err, json);
                }

                Ok(body.unwrap())
            }
            StatusCode::CREATED => {
                let json: Result<String, _> = response.text().await;
                let Ok(json) = json else {
                    bail!("Failed to read response body");
                };
                let body: Result<T, _> = serde_json::from_str(&json);
                if let Err(err) = body {
                    bail!("Failed to deserialize body {:?}: {}", err, json);
                }

                Ok(body.unwrap())
            }
            StatusCode::INTERNAL_SERVER_ERROR => {
                bail!("Internal Server Error");
            }
            StatusCode::SERVICE_UNAVAILABLE => {
                bail!("Service Unavailable");
            }
            StatusCode::UNAUTHORIZED => {
                bail!("Unauthorized");
            }
            StatusCode::BAD_REQUEST => {
                let error = response.text().await?;
                bail!(format!(
                    "Received bad request status. Request: {:?}. Response: {:?}",
                    request_json, error
                ));
            }
            s => {
                let error = response.text().await?;

                bail!(format!("Received response code: {s:?} error: {error:?}"));
            }
        }
    }
}

pub struct GateFiBuyAssetParams {
    pub amount: String,
    pub crypto: String,
    pub fiat: String,
    pub order_custom_id: String,
    pub payment_method: GateFiBuyAssetPaymentMethod,
    pub redirect_url: String,
    pub region: String,
    pub wallet_address: String,
}

pub enum GateFiBuyAssetPaymentMethod {
    DebitCreditCard,
    ApplePay,
    SepaBankTransfer,
    GbpBankTransfer,
    AchBankTransfer,
    Upi,
    Pix,
}

impl fmt::Display for GateFiBuyAssetPaymentMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GateFiBuyAssetPaymentMethod::DebitCreditCard => write!(f, "debit-credit-card"),
            GateFiBuyAssetPaymentMethod::ApplePay => write!(f, "apple-pay"),
            GateFiBuyAssetPaymentMethod::SepaBankTransfer => write!(f, "sepa-bank-transfer"),
            GateFiBuyAssetPaymentMethod::GbpBankTransfer => write!(f, "gbp-bank-transfer"),
            GateFiBuyAssetPaymentMethod::AchBankTransfer => write!(f, "ach-bank-transfer"),
            GateFiBuyAssetPaymentMethod::Upi => write!(f, "upi"),
            GateFiBuyAssetPaymentMethod::Pix => write!(f, "pix"),
        }
    }
}
