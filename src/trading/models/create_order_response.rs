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
pub struct CreateOrderResponse {
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "regulatory_information")]
    pub regulatory_information: Box<crate::trading::models::RegulatoryInformation>,
    #[serde(rename = "isin")]
    pub isin: String,
    #[serde(rename = "expires_at")]
    pub expires_at: String,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "quantity")]
    pub quantity: i32,
    #[serde(rename = "stop_price", skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<i32>,
    #[serde(rename = "limit_price", skip_serializing_if = "Option::is_none")]
    pub limit_price: Option<i32>,
    #[serde(rename = "venue", skip_serializing_if = "Option::is_none")]
    pub venue: Option<String>,
    #[serde(rename = "estimated_price")]
    pub estimated_price: i32,
    #[serde(rename = "estimated_price_total")]
    pub estimated_price_total: i32,
    #[serde(rename = "notes", skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "charge", skip_serializing_if = "Option::is_none")]
    pub charge: Option<i32>,
    #[serde(rename = "chargeable_at", skip_serializing_if = "Option::is_none")]
    pub chargeable_at: Option<String>,
    #[serde(rename = "key_creation_id", skip_serializing_if = "Option::is_none")]
    pub key_creation_id: Option<String>,
    #[serde(rename = "idempotency", skip_serializing_if = "Option::is_none")]
    pub idempotency: Option<String>,
}

impl CreateOrderResponse {
    pub fn new(created_at: String, id: String, status: String, regulatory_information: crate::trading::models::RegulatoryInformation, isin: String, expires_at: String, side: String, quantity: i32, estimated_price: i32, estimated_price_total: i32) -> CreateOrderResponse {
        CreateOrderResponse {
            created_at,
            id,
            status,
            regulatory_information: Box::new(regulatory_information),
            isin,
            expires_at,
            side,
            quantity,
            stop_price: None,
            limit_price: None,
            venue: None,
            estimated_price,
            estimated_price_total,
            notes: None,
            charge: None,
            chargeable_at: None,
            key_creation_id: None,
            idempotency: None,
        }
    }
}

