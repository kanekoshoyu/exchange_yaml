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
pub struct RepaymentInfo {
    #[serde(rename = "loanCoin")]
    pub loan_coin: String,
    #[serde(rename = "remainingPrincipal")]
    pub remaining_principal: String,
    #[serde(rename = "remainingInterest")]
    pub remaining_interest: String,
    #[serde(rename = "collateralCoin")]
    pub collateral_coin: String,
    #[serde(rename = "remainingCollateral")]
    pub remaining_collateral: String,
    #[serde(rename = "currentLTV")]
    pub current_ltv: String,
    #[serde(rename = "repayStatus")]
    pub repay_status: String,
}

impl RepaymentInfo {
    pub fn new(loan_coin: String, remaining_principal: String, remaining_interest: String, collateral_coin: String, remaining_collateral: String, current_ltv: String, repay_status: String) -> RepaymentInfo {
        RepaymentInfo {
            loan_coin,
            remaining_principal,
            remaining_interest,
            collateral_coin,
            remaining_collateral,
            current_ltv,
            repay_status,
        }
    }
}

