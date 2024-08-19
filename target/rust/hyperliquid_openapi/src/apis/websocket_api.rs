/*
 * Hyperliquid API
 *
 * Documentation for the Hyperliquid public API     ## **Rate limits** The following rate limits apply per IP address:   - All REST requests have a weight limit of 1200 per minute. All documented exchange API requests have a weight of 1. All documented info API requests have a weight of either 2 or 20; these limits can be found in the description for each info request in the Info endpoint section. All explorer API requests have a weight of 40.   - Maximum of 100 websocket connections   - Maximum of 1000 websocket subscriptions   - Maximum of 10 unique users across user-specific websocket subscriptions  - Maximum of 2000 inbound messages per minute across all websocket connections   - Use websockets for lowest latency realtime data. See the python SDK for a full-featured example.    ## **Address-based L1 Rate limits**    The L1 rate limiting logic will allow 1 requests per 1 USDC traded cumulatively since address inception.   Using an order value of 100 USDC, this only requires a fill rate of 1%.    Each address starts with an initial buffer of 10000 requests. When rate limited, an address will still be allowed one request every 10 seconds.  Cancels have cumulative limit min(limit + 100000, limit * 2) where limit is the default limit for other actions. This way, hitting the address-based rate limit will still allow open orders to be canceled.   Note that this rate limit only applies to L1 actions, not info requests.   ## **Batched Requests** A batched request with n orders (or cancels) is treated as one request for IP based rate limiting, but as n requests for address-based L1 rate limiting. 
 *
 * The version of the OpenAPI document: 1.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};


/// struct for typed errors of method [`ws_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WsPostError {
    Status400(models::Error),
    Status402(models::Error),
    Status404(models::Error),
    Status422(models::Error),
    Status500(models::GetInfoOrOrders500Response),
    UnknownValue(serde_json::Value),
}


/// The WebSocket API allows for real-time data streaming and serves as an alternative to HTTP requests on the Hyperliquid exchange. - **Mainnet URL:** `wss://api.hyperliquid.xyz/ws` - **Testnet URL:** `wss://api.hyperliquid-testnet.xyz/ws` 
pub async fn ws_post(configuration: &configuration::Configuration, ws_post_request: models::WsPostRequest) -> Result<models::WsPost200Response, Error<WsPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/ws", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&ws_post_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<WsPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

