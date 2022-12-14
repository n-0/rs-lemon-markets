/*
 * lemon.markets | Historical Market Data API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1da2ed0b5fefcf155fa143ba4b81c08c3c1c351c
 * 
 * Generated by: https://openapi-generator.tech
 */

/// SortType : An enumeration.

/// An enumeration.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SortType {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,

}

impl ToString for SortType {
    fn to_string(&self) -> String {
        match self {
            Self::Asc => String::from("asc"),
            Self::Desc => String::from("desc"),
        }
    }
}

impl Default for SortType {
    fn default() -> SortType {
        Self::Asc
    }
}




