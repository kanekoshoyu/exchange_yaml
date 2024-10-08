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
pub struct SapiV1MarginLeverageBracketGet200ResponseInnerBracketsInner {
    #[serde(rename = "leverage", skip_serializing_if = "Option::is_none")]
    pub leverage: Option<i32>,
    #[serde(rename = "maxDebt", skip_serializing_if = "Option::is_none")]
    pub max_debt: Option<f64>,
    #[serde(rename = "maintenanceMarginRate", skip_serializing_if = "Option::is_none")]
    pub maintenance_margin_rate: Option<f64>,
    #[serde(rename = "initialMarginRate", skip_serializing_if = "Option::is_none")]
    pub initial_margin_rate: Option<f64>,
    #[serde(rename = "fastNum", skip_serializing_if = "Option::is_none")]
    pub fast_num: Option<f64>,
}

impl SapiV1MarginLeverageBracketGet200ResponseInnerBracketsInner {
    pub fn new() -> SapiV1MarginLeverageBracketGet200ResponseInnerBracketsInner {
        SapiV1MarginLeverageBracketGet200ResponseInnerBracketsInner {
            leverage: None,
            max_debt: None,
            maintenance_margin_rate: None,
            initial_margin_rate: None,
            fast_num: None,
        }
    }
}

