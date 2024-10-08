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
pub struct ApiV3SorOrderPost200ResponseFillsInner {
    #[serde(rename = "matchType")]
    pub match_type: String,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "qty")]
    pub qty: String,
    #[serde(rename = "commission")]
    pub commission: String,
    #[serde(rename = "commissionAsset")]
    pub commission_asset: String,
    #[serde(rename = "tradeId")]
    pub trade_id: i64,
    #[serde(rename = "allocId")]
    pub alloc_id: i64,
}

impl ApiV3SorOrderPost200ResponseFillsInner {
    pub fn new(match_type: String, price: String, qty: String, commission: String, commission_asset: String, trade_id: i64, alloc_id: i64) -> ApiV3SorOrderPost200ResponseFillsInner {
        ApiV3SorOrderPost200ResponseFillsInner {
            match_type,
            price,
            qty,
            commission,
            commission_asset,
            trade_id,
            alloc_id,
        }
    }
}

