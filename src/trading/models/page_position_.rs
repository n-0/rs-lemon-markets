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
pub struct PagePosition {
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<crate::trading::models::Mode>,
    #[serde(rename = "results")]
    pub results: Vec<crate::trading::models::Position>,
    #[serde(rename = "previous", skip_serializing_if = "Option::is_none")]
    pub previous: Option<String>,
    #[serde(rename = "next", skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(rename = "total")]
    pub total: i32,
    #[serde(rename = "page")]
    pub page: i32,
    #[serde(rename = "pages")]
    pub pages: i32,
}

impl PagePosition {
    pub fn new(results: Vec<crate::trading::models::Position>, total: i32, page: i32, pages: i32) -> PagePosition {
        PagePosition {
            time: None,
            status: None,
            mode: None,
            results,
            previous: None,
            next: None,
            total,
            page,
            pages,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "ok")]
    Ok,
}

impl Default for Status {
    fn default() -> Status {
        Self::Ok
    }
}

