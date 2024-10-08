/*
 * Binance Public Spot API
 *
 * OpenAPI Specifications for the Binance Public Spot API  API documents:   - [https://github.com/binance/binance-spot-api-docs](https://github.com/binance/binance-spot-api-docs)   - [https://binance-docs.github.io/apidocs/spot/en](https://binance-docs.github.io/apidocs/spot/en)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SapiV1AccountApiRestrictionsGet200Response {
    #[serde(rename = "ipRestrict")]
    pub ip_restrict: bool,
    #[serde(rename = "createTime")]
    pub create_time: i64,
    /// This option authorizes this key to transfer funds between your master account and your sub account instantly
    #[serde(rename = "enableInternalTransfer")]
    pub enable_internal_transfer: bool,
    /// API Key created before your futures account opened does not support futures API service
    #[serde(rename = "enableFutures")]
    pub enable_futures: bool,
    /// API Key created before your activate portfolio margin does not support portfolio margin API service
    #[serde(rename = "enablePortfolioMarginTrading", skip_serializing_if = "Option::is_none")]
    pub enable_portfolio_margin_trading: Option<bool>,
    /// Authorizes this key to Vanilla options trading
    #[serde(rename = "enableVanillaOptions")]
    pub enable_vanilla_options: bool,
    /// Authorizes this key to be used for a dedicated universal transfer API to transfer multiple supported currencies. Each business's own transfer API rights are not affected by this authorization
    #[serde(rename = "permitsUniversalTransfer")]
    pub permits_universal_transfer: bool,
    #[serde(rename = "enableReading")]
    pub enable_reading: bool,
    #[serde(rename = "enableSpotAndMarginTrading")]
    pub enable_spot_and_margin_trading: bool,
    /// This option allows you to withdraw via API. You must apply the IP Access Restriction filter in order to enable withdrawals
    #[serde(rename = "enableWithdrawals")]
    pub enable_withdrawals: bool,
    /// This option can be adjusted after the Cross Margin account transfer is completed
    #[serde(rename = "enableMargin")]
    pub enable_margin: bool,
    /// Expiration time for spot and margin trading permission
    #[serde(rename = "tradingAuthorityExpirationTime")]
    pub trading_authority_expiration_time: i64,
}

impl SapiV1AccountApiRestrictionsGet200Response {
    pub fn new(ip_restrict: bool, create_time: i64, enable_internal_transfer: bool, enable_futures: bool, enable_vanilla_options: bool, permits_universal_transfer: bool, enable_reading: bool, enable_spot_and_margin_trading: bool, enable_withdrawals: bool, enable_margin: bool, trading_authority_expiration_time: i64) -> SapiV1AccountApiRestrictionsGet200Response {
        SapiV1AccountApiRestrictionsGet200Response {
            ip_restrict,
            create_time,
            enable_internal_transfer,
            enable_futures,
            enable_portfolio_margin_trading: None,
            enable_vanilla_options,
            permits_universal_transfer,
            enable_reading,
            enable_spot_and_margin_trading,
            enable_withdrawals,
            enable_margin,
            trading_authority_expiration_time,
        }
    }
}

