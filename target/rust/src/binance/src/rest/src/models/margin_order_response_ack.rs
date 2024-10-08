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
pub struct MarginOrderResponseAck {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "orderId")]
    pub order_id: i64,
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,
    #[serde(rename = "isIsolated")]
    pub is_isolated: bool,
    #[serde(rename = "transactTime")]
    pub transact_time: i64,
}

impl MarginOrderResponseAck {
    pub fn new(symbol: String, order_id: i64, client_order_id: String, is_isolated: bool, transact_time: i64) -> MarginOrderResponseAck {
        MarginOrderResponseAck {
            symbol,
            order_id,
            client_order_id,
            is_isolated,
            transact_time,
        }
    }
}

