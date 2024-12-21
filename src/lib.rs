const API_ROOT: &str = "http://ws.audioscrobbler.com/2.0/";
type Param<'a> = (&'a str, &'a str);

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
    fn build(&self, params: &[Param]) -> reqwest::RequestBuilder {
        let params = &[params, &[("api_key", &self.key), ("format", "json")]].concat();
        self.client.get(API_ROOT).query(params)
    }
}
