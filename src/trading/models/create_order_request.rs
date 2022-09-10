/*
 * lemon.markets | Trading API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: b996d5fa9acbe1796d08c2f8360172c63ac713ad
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateOrderRequest {
    #[serde(rename = "isin")]
    pub isin: String,
    #[serde(rename = "expires_at", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<serde_json::Value>,
    #[serde(rename = "side")]
    pub side: crate::trading::models::Sides,
    #[serde(rename = "quantity")]
    pub quantity: i32,
    #[serde(rename = "stop_price", skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<i32>,
    #[serde(rename = "limit_price", skip_serializing_if = "Option::is_none")]
    pub limit_price: Option<i32>,
    #[serde(rename = "venue", skip_serializing_if = "Option::is_none")]
    pub venue: Option<Box<crate::trading::models::Venue>>,
    #[serde(rename = "notes", skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "idempotency", skip_serializing_if = "Option::is_none")]
    pub idempotency: Option<String>,
}

impl CreateOrderRequest {
    pub fn new(isin: String, side: crate::trading::models::Sides, quantity: i32) -> CreateOrderRequest {
        CreateOrderRequest {
            isin,
            expires_at: None,
            side,
            quantity,
            stop_price: None,
            limit_price: None,
            venue: None,
            notes: None,
            idempotency: None,
        }
    }
}


