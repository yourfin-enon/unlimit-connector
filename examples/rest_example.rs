use rust_extensions::date_time::DateTimeAsMicroseconds;
use unlimit_connector::rest::config::GateFiApiConfig;
use unlimit_connector::rest::rest_client::{
    GateFiBuyAssetParams, GateFiBuyAssetPaymentMethod, GateFiRestClient,
};

#[tokio::main]
async fn main() {
    let client = GateFiRestClient::new(
        "test".to_owned(),
        "test".to_owned(),
        "test".to_owned(),
        GateFiApiConfig::sandbox());
    get_platform_config(&client).await;
    get_quote(&client).await;
    buy_asset(&client).await;
}

async fn get_quote(client: &GateFiRestClient) {
    let address = client
        .get_quote(
            "100",
            "BTC",
            "USD",
            &GateFiBuyAssetPaymentMethod::DebitCreditCard,
            "BR",
        )
        .await;
    println!("get_quote result: {address:?}");
}

async fn buy_asset(client: &GateFiRestClient) {
    let params = GateFiBuyAssetParams {
        amount: "100".to_string(),
        crypto: "BTC".to_string(),
        fiat: "USD".to_string(),
        order_custom_id: format!("test-{}", DateTimeAsMicroseconds::now().unix_microseconds),
        payment_method: GateFiBuyAssetPaymentMethod::DebitCreditCard,
        redirect_url: "google.com".to_string(),
        region: "BR".to_string(),
        wallet_address: "2Mxsqy9d6LuW2VYQPsojmPWXaRznMQ7Nifr2".to_string(),
    };
    let result = client.buy_asset(params).await;
    println!("buy_asset result: {result:?}");
}

async fn get_platform_config(client: &GateFiRestClient) {
    let result = client.get_platform_config().await;
    println!("get_config result: {result:?}");
}
