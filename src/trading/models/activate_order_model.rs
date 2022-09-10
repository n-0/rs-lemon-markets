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
pub struct ActivateOrderModel {
    #[serde(rename = "pin", skip_serializing_if = "Option::is_none")]
    pub pin: Option<String>,
}

impl ActivateOrderModel {
    pub fn new() -> ActivateOrderModel {
        ActivateOrderModel {
            pin: None,
        }
    }
}


