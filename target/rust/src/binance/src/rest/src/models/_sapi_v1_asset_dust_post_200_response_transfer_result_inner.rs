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
pub struct SapiV1AssetDustPost200ResponseTransferResultInner {
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "fromAsset")]
    pub from_asset: String,
    #[serde(rename = "operateTime")]
    pub operate_time: i64,
    #[serde(rename = "serviceChargeAmount")]
    pub service_charge_amount: String,
    #[serde(rename = "tranId")]
    pub tran_id: i64,
    #[serde(rename = "transferedAmount")]
    pub transfered_amount: String,
}

impl SapiV1AssetDustPost200ResponseTransferResultInner {
    pub fn new(amount: String, from_asset: String, operate_time: i64, service_charge_amount: String, tran_id: i64, transfered_amount: String) -> SapiV1AssetDustPost200ResponseTransferResultInner {
        SapiV1AssetDustPost200ResponseTransferResultInner {
            amount,
            from_asset,
            operate_time,
            service_charge_amount,
            tran_id,
            transfered_amount,
        }
    }
}

