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
pub struct SapiV1SimpleEarnAccountGet200Response {
    #[serde(rename = "totalAmountInBTC")]
    pub total_amount_in_btc: String,
    #[serde(rename = "totalAmountInUSDT")]
    pub total_amount_in_usdt: String,
    #[serde(rename = "totalFlexibleAmountInBTC")]
    pub total_flexible_amount_in_btc: String,
    #[serde(rename = "totalFlexibleAmountInUSDT")]
    pub total_flexible_amount_in_usdt: String,
    #[serde(rename = "totalLockedInBTC")]
    pub total_locked_in_btc: String,
    #[serde(rename = "totalLockedInUSDT")]
    pub total_locked_in_usdt: String,
}

impl SapiV1SimpleEarnAccountGet200Response {
    pub fn new(total_amount_in_btc: String, total_amount_in_usdt: String, total_flexible_amount_in_btc: String, total_flexible_amount_in_usdt: String, total_locked_in_btc: String, total_locked_in_usdt: String) -> SapiV1SimpleEarnAccountGet200Response {
        SapiV1SimpleEarnAccountGet200Response {
            total_amount_in_btc,
            total_amount_in_usdt,
            total_flexible_amount_in_btc,
            total_flexible_amount_in_usdt,
            total_locked_in_btc,
            total_locked_in_usdt,
        }
    }
}

