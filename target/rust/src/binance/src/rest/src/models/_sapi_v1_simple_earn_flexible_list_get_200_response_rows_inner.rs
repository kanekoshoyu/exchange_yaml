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
pub struct SapiV1SimpleEarnFlexibleListGet200ResponseRowsInner {
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "latestAnnualPercentageRate")]
    pub latest_annual_percentage_rate: String,
    #[serde(rename = "tierAnnualPercentageRate")]
    pub tier_annual_percentage_rate: Box<models::SapiV1SimpleEarnFlexibleListGet200ResponseRowsInnerTierAnnualPercentageRate>,
    #[serde(rename = "airDropPercentageRate")]
    pub air_drop_percentage_rate: String,
    #[serde(rename = "canPurchase")]
    pub can_purchase: bool,
    #[serde(rename = "canRedeem")]
    pub can_redeem: bool,
    #[serde(rename = "isSoldOut")]
    pub is_sold_out: bool,
    #[serde(rename = "hot")]
    pub hot: bool,
    #[serde(rename = "minPurchaseAmount")]
    pub min_purchase_amount: String,
    #[serde(rename = "productId")]
    pub product_id: String,
    #[serde(rename = "subscriptionStartTime")]
    pub subscription_start_time: String,
    #[serde(rename = "status")]
    pub status: String,
}

impl SapiV1SimpleEarnFlexibleListGet200ResponseRowsInner {
    pub fn new(asset: String, latest_annual_percentage_rate: String, tier_annual_percentage_rate: models::SapiV1SimpleEarnFlexibleListGet200ResponseRowsInnerTierAnnualPercentageRate, air_drop_percentage_rate: String, can_purchase: bool, can_redeem: bool, is_sold_out: bool, hot: bool, min_purchase_amount: String, product_id: String, subscription_start_time: String, status: String) -> SapiV1SimpleEarnFlexibleListGet200ResponseRowsInner {
        SapiV1SimpleEarnFlexibleListGet200ResponseRowsInner {
            asset,
            latest_annual_percentage_rate,
            tier_annual_percentage_rate: Box::new(tier_annual_percentage_rate),
            air_drop_percentage_rate,
            can_purchase,
            can_redeem,
            is_sold_out,
            hot,
            min_purchase_amount,
            product_id,
            subscription_start_time,
            status,
        }
    }
}

