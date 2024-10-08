/*
 * Binance Public Spot API
 *
 * OpenAPI Specifications for the Binance Public Spot API  API documents:   - [https://github.com/binance/binance-spot-api-docs](https://github.com/binance/binance-spot-api-docs)   - [https://binance-docs.github.io/apidocs/spot/en](https://binance-docs.github.io/apidocs/spot/en)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};


/// struct for typed errors of method [`sapi_v1_portfolio_account_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1PortfolioAccountGetError {
    Status400(models::Error),
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sapi_v1_portfolio_asset_collection_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1PortfolioAssetCollectionPostError {
    Status400(models::Error),
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sapi_v1_portfolio_asset_index_price_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1PortfolioAssetIndexPriceGetError {
    Status400(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sapi_v1_portfolio_auto_collection_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1PortfolioAutoCollectionPostError {
    Status400(models::Error),
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sapi_v1_portfolio_bnb_transfer_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1PortfolioBnbTransferPostError {
    Status400(models::Error),
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sapi_v1_portfolio_collateral_rate_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1PortfolioCollateralRateGetError {
    Status400(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sapi_v1_portfolio_interest_history_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1PortfolioInterestHistoryGetError {
    Status400(models::Error),
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sapi_v1_portfolio_margin_asset_leverage_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1PortfolioMarginAssetLeverageGetError {
    Status400(models::Error),
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sapi_v1_portfolio_pm_loan_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1PortfolioPmLoanGetError {
    Status400(models::Error),
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sapi_v1_portfolio_repay_futures_negative_balance_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1PortfolioRepayFuturesNegativeBalancePostError {
    Status400(models::Error),
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sapi_v1_portfolio_repay_futures_switch_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1PortfolioRepayFuturesSwitchGetError {
    Status400(models::Error),
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sapi_v1_portfolio_repay_futures_switch_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1PortfolioRepayFuturesSwitchPostError {
    Status400(models::Error),
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sapi_v1_portfolio_repay_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1PortfolioRepayPostError {
    Status400(models::Error),
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}


/// Get the account info  'Weight(IP): 1'
pub async fn sapi_v1_portfolio_account_get(configuration: &configuration::Configuration, timestamp: i64, signature: &str, recv_window: Option<i64>) -> Result<models::SapiV1PortfolioAccountGet200Response, Error<SapiV1PortfolioAccountGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/sapi/v1/portfolio/account", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = recv_window {
        local_var_req_builder = local_var_req_builder.query(&[("recvWindow", &local_var_str.to_string())]);
    }
    local_var_req_builder = local_var_req_builder.query(&[("timestamp", &timestamp.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("signature", &signature.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-MBX-APIKEY", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SapiV1PortfolioAccountGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Transfers specific asset from Futures Account to Margin account  Weight(IP): 60
pub async fn sapi_v1_portfolio_asset_collection_post(configuration: &configuration::Configuration, asset: &str, timestamp: i64, signature: &str, recv_window: Option<i64>) -> Result<models::SapiV1PortfolioAutoCollectionPost200Response, Error<SapiV1PortfolioAssetCollectionPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/sapi/v1/portfolio/asset-collection", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("asset", &asset.to_string())]);
    if let Some(ref local_var_str) = recv_window {
        local_var_req_builder = local_var_req_builder.query(&[("recvWindow", &local_var_str.to_string())]);
    }
    local_var_req_builder = local_var_req_builder.query(&[("timestamp", &timestamp.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("signature", &signature.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-MBX-APIKEY", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SapiV1PortfolioAssetCollectionPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Query Portfolio Margin Asset Index Price  Weight(IP): - 1 if send asset - 50 if not send asset
pub async fn sapi_v1_portfolio_asset_index_price_get(configuration: &configuration::Configuration, asset: Option<&str>) -> Result<Vec<models::SapiV1PortfolioAssetIndexPriceGet200ResponseInner>, Error<SapiV1PortfolioAssetIndexPriceGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/sapi/v1/portfolio/asset-index-price", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = asset {
        local_var_req_builder = local_var_req_builder.query(&[("asset", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-MBX-APIKEY", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SapiV1PortfolioAssetIndexPriceGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Transfers all assets from Futures Account to Margin account  Weight(IP): 1500
pub async fn sapi_v1_portfolio_auto_collection_post(configuration: &configuration::Configuration, timestamp: i64, signature: &str, recv_window: Option<i64>) -> Result<models::SapiV1PortfolioAutoCollectionPost200Response, Error<SapiV1PortfolioAutoCollectionPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/sapi/v1/portfolio/auto-collection", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = recv_window {
        local_var_req_builder = local_var_req_builder.query(&[("recvWindow", &local_var_str.to_string())]);
    }
    local_var_req_builder = local_var_req_builder.query(&[("timestamp", &timestamp.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("signature", &signature.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-MBX-APIKEY", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SapiV1PortfolioAutoCollectionPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// BNB transfer can be between Margin Account and USDM Account  Weight(IP): 1500
pub async fn sapi_v1_portfolio_bnb_transfer_post(configuration: &configuration::Configuration, transfer_side: &str, amount: f64, timestamp: i64, signature: &str, recv_window: Option<i64>) -> Result<models::SapiV1MarginBorrowRepayPost200Response, Error<SapiV1PortfolioBnbTransferPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/sapi/v1/portfolio/bnb-transfer", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("transferSide", &transfer_side.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("amount", &amount.to_string())]);
    if let Some(ref local_var_str) = recv_window {
        local_var_req_builder = local_var_req_builder.query(&[("recvWindow", &local_var_str.to_string())]);
    }
    local_var_req_builder = local_var_req_builder.query(&[("timestamp", &timestamp.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("signature", &signature.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-MBX-APIKEY", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SapiV1PortfolioBnbTransferPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Portfolio Margin Collateral Rate.  Weight(IP): 50
pub async fn sapi_v1_portfolio_collateral_rate_get(configuration: &configuration::Configuration, ) -> Result<Vec<models::SapiV1PortfolioCollateralRateGet200ResponseInner>, Error<SapiV1PortfolioCollateralRateGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/sapi/v1/portfolio/collateralRate", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-MBX-APIKEY", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SapiV1PortfolioCollateralRateGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Query interest history of negative balance for portfolio margin.  Weight(IP): 50
pub async fn sapi_v1_portfolio_interest_history_get(configuration: &configuration::Configuration, asset: &str, timestamp: i64, signature: &str, start_time: Option<i64>, end_time: Option<i64>, size: Option<i32>, recv_window: Option<i64>) -> Result<Vec<models::SapiV1PortfolioInterestHistoryGet200ResponseInner>, Error<SapiV1PortfolioInterestHistoryGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/sapi/v1/portfolio/interest-history", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("asset", &asset.to_string())]);
    if let Some(ref local_var_str) = start_time {
        local_var_req_builder = local_var_req_builder.query(&[("startTime", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = end_time {
        local_var_req_builder = local_var_req_builder.query(&[("endTime", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = size {
        local_var_req_builder = local_var_req_builder.query(&[("size", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = recv_window {
        local_var_req_builder = local_var_req_builder.query(&[("recvWindow", &local_var_str.to_string())]);
    }
    local_var_req_builder = local_var_req_builder.query(&[("timestamp", &timestamp.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("signature", &signature.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-MBX-APIKEY", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SapiV1PortfolioInterestHistoryGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Weight(IP): 50
pub async fn sapi_v1_portfolio_margin_asset_leverage_get(configuration: &configuration::Configuration, ) -> Result<Vec<models::SapiV1PortfolioMarginAssetLeverageGet200ResponseInner>, Error<SapiV1PortfolioMarginAssetLeverageGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/sapi/v1/portfolio/margin-asset-leverage", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SapiV1PortfolioMarginAssetLeverageGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Query Portfolio Margin Bankruptcy Loan Amount.  Weight(UID): 500
pub async fn sapi_v1_portfolio_pm_loan_get(configuration: &configuration::Configuration, timestamp: i64, signature: &str, recv_window: Option<i64>) -> Result<models::SapiV1PortfolioPmLoanGet200Response, Error<SapiV1PortfolioPmLoanGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/sapi/v1/portfolio/pmLoan", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = recv_window {
        local_var_req_builder = local_var_req_builder.query(&[("recvWindow", &local_var_str.to_string())]);
    }
    local_var_req_builder = local_var_req_builder.query(&[("timestamp", &timestamp.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("signature", &signature.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-MBX-APIKEY", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SapiV1PortfolioPmLoanGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Repay futures Negative Balance  Weight(IP): 1500
pub async fn sapi_v1_portfolio_repay_futures_negative_balance_post(configuration: &configuration::Configuration, timestamp: i64, signature: &str, recv_window: Option<i64>) -> Result<models::SapiV1PortfolioAutoCollectionPost200Response, Error<SapiV1PortfolioRepayFuturesNegativeBalancePostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/sapi/v1/portfolio/repay-futures-negative-balance", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = recv_window {
        local_var_req_builder = local_var_req_builder.query(&[("recvWindow", &local_var_str.to_string())]);
    }
    local_var_req_builder = local_var_req_builder.query(&[("timestamp", &timestamp.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("signature", &signature.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-MBX-APIKEY", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SapiV1PortfolioRepayFuturesNegativeBalancePostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Query Auto-repay-futures Status  Weight(IP): 30
pub async fn sapi_v1_portfolio_repay_futures_switch_get(configuration: &configuration::Configuration, timestamp: i64, signature: &str, recv_window: Option<i64>) -> Result<models::SapiV1PortfolioRepayFuturesSwitchGet200Response, Error<SapiV1PortfolioRepayFuturesSwitchGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/sapi/v1/portfolio/repay-futures-switch", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = recv_window {
        local_var_req_builder = local_var_req_builder.query(&[("recvWindow", &local_var_str.to_string())]);
    }
    local_var_req_builder = local_var_req_builder.query(&[("timestamp", &timestamp.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("signature", &signature.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-MBX-APIKEY", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SapiV1PortfolioRepayFuturesSwitchGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Change Auto-repay-futures Status  Weight(IP): 1500
pub async fn sapi_v1_portfolio_repay_futures_switch_post(configuration: &configuration::Configuration, auto_repay: bool, timestamp: i64, signature: &str, recv_window: Option<i64>) -> Result<models::SapiV1PortfolioAutoCollectionPost200Response, Error<SapiV1PortfolioRepayFuturesSwitchPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/sapi/v1/portfolio/repay-futures-switch", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("autoRepay", &auto_repay.to_string())]);
    if let Some(ref local_var_str) = recv_window {
        local_var_req_builder = local_var_req_builder.query(&[("recvWindow", &local_var_str.to_string())]);
    }
    local_var_req_builder = local_var_req_builder.query(&[("timestamp", &timestamp.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("signature", &signature.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-MBX-APIKEY", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SapiV1PortfolioRepayFuturesSwitchPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Repay Portfolio Margin Bankruptcy Loan.  Weight(UID): 3000
pub async fn sapi_v1_portfolio_repay_post(configuration: &configuration::Configuration, timestamp: i64, signature: &str, from: Option<&str>, recv_window: Option<i64>) -> Result<models::SapiV1PortfolioRepayPost200Response, Error<SapiV1PortfolioRepayPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/sapi/v1/portfolio/repay", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = from {
        local_var_req_builder = local_var_req_builder.query(&[("from", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = recv_window {
        local_var_req_builder = local_var_req_builder.query(&[("recvWindow", &local_var_str.to_string())]);
    }
    local_var_req_builder = local_var_req_builder.query(&[("timestamp", &timestamp.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("signature", &signature.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-MBX-APIKEY", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SapiV1PortfolioRepayPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

