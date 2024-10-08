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
pub struct SapiV1UserDataStreamIsolatedPost200Response {
    #[serde(rename = "listenKey")]
    pub listen_key: String,
}

impl SapiV1UserDataStreamIsolatedPost200Response {
    pub fn new(listen_key: String) -> SapiV1UserDataStreamIsolatedPost200Response {
        SapiV1UserDataStreamIsolatedPost200Response {
            listen_key,
        }
    }
}

