mod raw;
use crate::shared::SizedImages;
use std::convert::From;

use crate::from_raw;
use raw::{RegUsize, URBool, URUsize};

#[derive(Debug)]
pub struct UserInfo {
    pub name: String,
    pub realname: String,
    pub age: usize,
    pub country: String,
    pub gender: String,
    pub subscriber: bool,
    pub bootstrap: usize,
    pub playlists: usize,
    pub playcount: usize,
    pub artist_count: usize,
    pub album_count: usize,
    pub track_count: usize,
    pub image: SizedImages,
    pub registered: usize,
    pub url: String,
}

impl From<raw::User> for UserInfo {
    fn from(item: raw::User) -> Self {
        from_raw! {
            item,
            {
                name,
                realname,
                country,
                gender,
                image,
                url
            },
            {
                age = item.aged(),
                subscriber = item.is_pro(),
                bootstrap = item.bootstraps(),
                playlists = item.lists(),
                playcount = item.plays(),
                artist_count = item.artists(),
                album_count = item.albums(),
                track_count = item.tracks(),
                registered = item.registered.time()
            }
        }
    }
}

use reqwest::Method;

use std::collections::HashMap;
pub struct User {
    name: String,
    pub params: HashMap<String, String>,
}

use super::Client;
impl User {
    pub fn new(name: String) -> Self {
        Self {
            name,
            params: HashMap::new(),
        }
    }
    pub async fn get_info(&self, client: &Client) -> anyhow::Result<UserInfo> {
        let r = client
            .build(Method::GET, "user.getInfo")
            .query(&[("user", self.name.as_str())]);
        let i: UserInfo = r.send().await?.json::<raw::Raw>().await?.user.into();
        Ok(i)
    }
    // TODO: pager
    pub async fn get_friends(&self, client: &Client) -> anyhow::Result<Vec<String>> {
        let r = client
            .build(Method::GET, "user.getFriends")
            .query(&[("user", self.name.as_str())])
            .query(&self.params);
        let f = r
            .send()
            .await?
            .json::<raw::Friends>()
            .await?
            .friends
            .user
            .into_iter()
            .map(|f| f.name)
            .collect::<Vec<String>>();
        Ok(f)
    }
}
