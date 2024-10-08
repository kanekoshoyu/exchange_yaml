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
pub struct SapiV1FiatOrdersGet200ResponseDataInner {
    #[serde(rename = "orderNo")]
    pub order_no: String,
    #[serde(rename = "fiatCurrency")]
    pub fiat_currency: String,
    #[serde(rename = "indicatedAmount")]
    pub indicated_amount: String,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "totalFee")]
    pub total_fee: String,
    #[serde(rename = "method")]
    pub method: String,
    /// Processing, Failed, Successful, Finished, Refunding, Refunded, Refund Failed, Order Partial credit Stopped
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "createTime")]
    pub create_time: i64,
    #[serde(rename = "updateTime")]
    pub update_time: i64,
}

impl SapiV1FiatOrdersGet200ResponseDataInner {
    pub fn new(order_no: String, fiat_currency: String, indicated_amount: String, amount: String, total_fee: String, method: String, status: String, create_time: i64, update_time: i64) -> SapiV1FiatOrdersGet200ResponseDataInner {
        SapiV1FiatOrdersGet200ResponseDataInner {
            order_no,
            fiat_currency,
            indicated_amount,
            amount,
            total_fee,
            method,
            status,
            create_time,
            update_time,
        }
    }
}

