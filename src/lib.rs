const API_ROOT: &str = "http://ws.audioscrobbler.com/2.0/";

pub mod album;
pub mod artist;
pub mod shared;
pub mod tag;
pub mod user;
mod utils;
pub struct Client {
    key: String,
    client: reqwest::Client,
}
impl std::convert::From<&str> for Client {
    fn from(item: &str) -> Self {
        Self {
            key: item.to_owned(),
            client: reqwest::Client::new(),
        }
    }
}

impl Client {
    fn build(&self, http: reqwest::Method, method: &str) -> reqwest::RequestBuilder {
        self.client
            .request(http, API_ROOT)
            .query(&[
                ("api_key", self.key.as_str()),
                ("method", method),
                ("format", "json")
            ])
    }
}
