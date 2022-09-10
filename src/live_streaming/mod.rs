pub mod quotes;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}


#[derive(Debug)]
struct Custom {}

impl std::fmt::Display for Custom {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "That didnt work")
    }
}

#[derive(Debug)]
pub enum QuoteError {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Ably(ably::ErrorInfo),
    Custom
}

