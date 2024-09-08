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
pub struct SapiV1DciProductSubscribePost200Response {
    #[serde(rename = "positionId")]
    pub position_id: i64,
    #[serde(rename = "investCoin")]
    pub invest_coin: String,
    #[serde(rename = "exercisedCoin")]
    pub exercised_coin: String,
    #[serde(rename = "subscriptionAmount")]
    pub subscription_amount: String,
    #[serde(rename = "duration")]
    pub duration: i32,
    /// STANDARD, ADVANCED, this field won't display when autocompound is set to None
    #[serde(rename = "autoCompoundPlan")]
    pub auto_compound_plan: String,
    #[serde(rename = "strikePrice")]
    pub strike_price: String,
    #[serde(rename = "settleDate")]
    pub settle_date: i64,
    #[serde(rename = "purchaseStatus")]
    pub purchase_status: String,
    #[serde(rename = "apr")]
    pub apr: String,
    #[serde(rename = "orderId")]
    pub order_id: i64,
    #[serde(rename = "purchaseTime")]
    pub purchase_time: i64,
    #[serde(rename = "optionType", skip_serializing_if = "Option::is_none")]
    pub option_type_double_quote: Option<String>,
}

impl SapiV1DciProductSubscribePost200Response {
    pub fn new(position_id: i64, invest_coin: String, exercised_coin: String, subscription_amount: String, duration: i32, auto_compound_plan: String, strike_price: String, settle_date: i64, purchase_status: String, apr: String, order_id: i64, purchase_time: i64) -> SapiV1DciProductSubscribePost200Response {
        SapiV1DciProductSubscribePost200Response {
            position_id,
            invest_coin,
            exercised_coin,
            subscription_amount,
            duration,
            auto_compound_plan,
            strike_price,
            settle_date,
            purchase_status,
            apr,
            order_id,
            purchase_time,
            option_type_double_quote: None,
        }
    }
}

