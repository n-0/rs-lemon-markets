/*
 * lemon.markets | Historical Market Data API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1da2ed0b5fefcf155fa143ba4b81c08c3c1c351c
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FromDate {
}

impl FromDate {
    pub fn new() -> FromDate {
        FromDate {
        }
    }
}

/*
impl ToString for FromDate {
    fn to_string(&self) -> String {
        match self {
            Self::Latest => String::from("latest"),
        }
    }
}

impl Default for FromDate {
    fn default() -> FromDate {
        Self::Latest
    }
}
*/
