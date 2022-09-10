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
pub struct ValidationError {
    #[serde(rename = "loc")]
    pub loc: Vec<crate::trading::models::LocationInner>,
    #[serde(rename = "msg")]
    pub msg: String,
    #[serde(rename = "type")]
    pub _type: String,
}

impl ValidationError {
    pub fn new(loc: Vec<crate::trading::models::LocationInner>, msg: String, _type: String) -> ValidationError {
        ValidationError {
            loc,
            msg,
            _type,
        }
    }
}

