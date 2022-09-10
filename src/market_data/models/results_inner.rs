/*
 * lemon.markets | Historical Market Data API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1da2ed0b5fefcf155fa143ba4b81c08c3c1c351c
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ResultsInner {
    #[serde(rename = "isin")]
    pub isin: String,
    #[serde(rename = "o")]
    pub o: f32,
    #[serde(rename = "h")]
    pub h: f32,
    #[serde(rename = "l")]
    pub l: f32,
    #[serde(rename = "c")]
    pub c: f32,
    #[serde(rename = "v")]
    pub v: i32,
    #[serde(rename = "pbv")]
    pub pbv: f32,
    #[serde(rename = "t")]
    pub t: Box<String>,
    #[serde(rename = "mic")]
    pub mic: String,
}

impl ResultsInner {
    pub fn new(isin: String, o: f32, h: f32, l: f32, c: f32, v: i32, pbv: f32, t: String, mic: String) -> ResultsInner {
        ResultsInner {
            isin,
            o,
            h,
            l,
            c,
            v,
            pbv,
            t: Box::new(t),
            mic,
        }
    }
}


