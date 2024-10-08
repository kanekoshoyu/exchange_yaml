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
pub struct SapiV1CapitalConfigGetallGet200ResponseInnerNetworkListInner {
    #[serde(rename = "addressRegex")]
    pub address_regex: String,
    #[serde(rename = "coin")]
    pub coin: String,
    /// shown only when \"depositEnable\" is false.
    #[serde(rename = "depositDesc")]
    pub deposit_desc: String,
    #[serde(rename = "depositEnable")]
    pub deposit_enable: bool,
    #[serde(rename = "isDefault")]
    pub is_default: bool,
    #[serde(rename = "memoRegex")]
    pub memo_regex: String,
    /// min number for balance confirmation.
    #[serde(rename = "minConfirm")]
    pub min_confirm: i64,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "network")]
    pub network: String,
    #[serde(rename = "specialTips")]
    pub special_tips: String,
    /// confirmation number for balance unlock.
    #[serde(rename = "unLockConfirm")]
    pub un_lock_confirm: i64,
    /// shown only when \"withdrawEnable\" is false
    #[serde(rename = "withdrawDesc")]
    pub withdraw_desc: String,
    #[serde(rename = "withdrawEnable")]
    pub withdraw_enable: bool,
    #[serde(rename = "withdrawFee")]
    pub withdraw_fee: String,
    #[serde(rename = "withdrawIntegerMultiple")]
    pub withdraw_integer_multiple: String,
    #[serde(rename = "withdrawMax")]
    pub withdraw_max: String,
    #[serde(rename = "withdrawMin")]
    pub withdraw_min: String,
    #[serde(rename = "sameAddress")]
    pub same_address: bool,
}

impl SapiV1CapitalConfigGetallGet200ResponseInnerNetworkListInner {
    pub fn new(address_regex: String, coin: String, deposit_desc: String, deposit_enable: bool, is_default: bool, memo_regex: String, min_confirm: i64, name: String, network: String, special_tips: String, un_lock_confirm: i64, withdraw_desc: String, withdraw_enable: bool, withdraw_fee: String, withdraw_integer_multiple: String, withdraw_max: String, withdraw_min: String, same_address: bool) -> SapiV1CapitalConfigGetallGet200ResponseInnerNetworkListInner {
        SapiV1CapitalConfigGetallGet200ResponseInnerNetworkListInner {
            address_regex,
            coin,
            deposit_desc,
            deposit_enable,
            is_default,
            memo_regex,
            min_confirm,
            name,
            network,
            special_tips,
            un_lock_confirm,
            withdraw_desc,
            withdraw_enable,
            withdraw_fee,
            withdraw_integer_multiple,
            withdraw_max,
            withdraw_min,
            same_address,
        }
    }
}

