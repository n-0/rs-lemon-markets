use ably::rest::Client;
use futures::stream::StreamExt;

pub fn create_isins_message(isins: Vec<String>) -> String {
    format!("isins {}", isins.join(","))
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LemonMarketLiveStreamingAuth {
    pub expires_at: i64,
    pub token: String,
    pub user_id: String
}
pub struct LemonMarketLSClient {
    pub rest_client: Option<ably::rest::Rest>,
    pub auth: Option<LemonMarketLiveStreamingAuth>,
}

pub async fn create_live_streaming_client() -> Result<LemonMarketLSClient, super::QuoteError<>> {
    let paper_trading_key = std::env::var_os("KEY_PAPER_TRADING").unwrap().into_string().unwrap();
    let url_live_streaming = std::env::var_os("URL_LIVE_STREAMING").unwrap().into_string().unwrap();
    let mut auth_headers = reqwest::header::HeaderMap::new();
    auth_headers.insert("Authorization", reqwest::header::HeaderValue::from_str(format!("Bearer {}", paper_trading_key).as_str()).unwrap());

    let client = reqwest::ClientBuilder::new().default_headers(auth_headers).build();
    match client {
        Ok(client) => {
            let auth_req = client.post(format!("{}/auth", url_live_streaming)).send().await;
            match auth_req {
                Ok(auth_res) => {
                    let auth_question = auth_res.text().await;
                    match auth_question {
                        Ok(auth_question) => {
                            let auth = serde_json::from_str::<LemonMarketLiveStreamingAuth>(&auth_question);
                            match auth {
                                Ok(auth) => {
                                    let ably_client = ably::Rest::from(auth.token.clone());
                                    let client = LemonMarketLSClient {
                                        rest_client: Some(ably_client),
                                        auth: Some(auth),
                                    };
                                    return Ok(client);
                                }
                                Err(e) => {
                                    return Err(super::QuoteError::Serde(e));
                                }
                            }
                        }
                        Err(e) => {
                            return Err(super::QuoteError::Reqwest(e));
                        }
                    }
                },
                Err(e) => {
                    return Err(super::QuoteError::Reqwest(e));
                }
            }
        },
        Err(e) => {
            return Err(super::QuoteError::Reqwest(e));
        }
    }
    //let auth_url = format!("{}/auth", url_live_streaming).parse().unwrap();
    //ably::ClientOptions::new().auth_url(auth_url).key(paper_trading_key).client()
    //Err(super::QuoteError::Custom)
}

pub struct ChannelCreation {
    pub client: LemonMarketLSClient,
    pub auth_channel_messages: tokio::sync::mpsc::Receiver<String>,
    pub sub_channel: ably::rest::Channel,
}

pub async fn create_channel(mut client: LemonMarketLSClient) -> Result<ChannelCreation, super::QuoteError> {
    let mut auth_tokio_channel: Option<tokio::sync::mpsc::Receiver<String>> = None;
    let (auth_tx, auth_tr) = tokio::sync::mpsc::channel(1000);
    let mut subs_channel: Option<ably::rest::Channel> = None;
    if let Some(auth) = client.auth.as_ref() {
        if let Some(rest_client) = client.rest_client.as_ref() {
            let auth_channel = rest_client.channels.get(auth.user_id.clone());
            tokio::spawn(async move {
                let mut pages = auth_channel.history().pages();
                while let Some(Ok(page)) = pages.next().await {
                    for msg in page.items().await {
                        for msg in msg {
                            match msg.data {
                                ably::rest::Data::String(v) => {
                                    auth_tx.send(v).await;
                                },
                                ably::rest::Data::JSON(v) => {
                                    auth_tx.send(v.to_string()).await;
                                },
                                _ => (),
                            };
                        }
                    }
                };
            });
            auth_tokio_channel = Some(auth_tr);
            subs_channel = Some(rest_client.channels.get(format!("{}.subscriptions", auth.user_id.clone())));
        }
    }
    if auth_tokio_channel.is_some() && subs_channel.is_some() {
        return Ok(ChannelCreation {
            client,
            auth_channel_messages: auth_tokio_channel.unwrap(),
            sub_channel: subs_channel.unwrap(),
        })
    }
    Err(super::QuoteError::Custom)
}

pub async fn subscribe_to_quotes(isins: Vec<String>, channel: ably::rest::Channel) 
    -> Result<tokio::sync::mpsc::Receiver<String>, super::QuoteError> {
    let subs = create_isins_message(isins);
    match channel.publish().string(subs).send().await {
        Ok(_) => {
            let (tx, tr) = tokio::sync::mpsc::channel(1000);
            tokio::spawn(async move {
                let mut pages = channel.history().pages();
                while let Some(Ok(page)) = pages.next().await {
                    for msg in page.items().await {
                        for msg in msg {
                            println!("{:#?}", msg.data);
                            match msg.data {
                                ably::rest::Data::String(v) => {
                                    tx.send(v).await;
                                },
                                ably::rest::Data::JSON(v) => {
                                    tx.send(v.to_string()).await;
                                },
                                _ => (),
                            };
                        }
                    }
                };
            });
            return Ok(tr);
        },
        Err(e) => {
            return Err(super::QuoteError::Ably(e));
        }
    }
}
