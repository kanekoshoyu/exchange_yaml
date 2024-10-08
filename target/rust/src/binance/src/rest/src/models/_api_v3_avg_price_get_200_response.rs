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
pub struct ApiV3AvgPriceGet200Response {
    /// Average price interval (in minutes)
    #[serde(rename = "mins")]
    pub mins: i64,
    /// Average price
    #[serde(rename = "price")]
    pub price: String,
    /// Last trade time
    #[serde(rename = "closeTime")]
    pub close_time: i64,
}

impl ApiV3AvgPriceGet200Response {
    pub fn new(mins: i64, price: String, close_time: i64) -> ApiV3AvgPriceGet200Response {
        ApiV3AvgPriceGet200Response {
            mins,
            price,
            close_time,
        }
    }
}

