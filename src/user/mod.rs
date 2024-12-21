mod raw;
use raw::SizedImages;
use std::convert::From;

use crate::from_raw;
use raw::{URBool, URUsize, RegUsize};


#[allow(dead_code)]
#[derive(Debug)]
pub struct UserInfo {
    name: String,
    realname: String,
    age: usize,
    country: String,
    gender: String,
    subscriber: bool,
    bootstrap: usize,
    playlists: usize,
    playcount: usize,
    artist_count: usize,
    album_count: usize,
    track_count: usize,
    image: SizedImages,
    registered: usize,
    url: String,
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

use super::Client;
impl UserInfo {
    pub async fn get(client: &Client, username: &str) -> anyhow::Result<Self> {
        let get_info_params = &[("method", "user.getInfo"), ("user", username)];
        let info: UserInfo = client
            .build(get_info_params)
            .send()
            .await?
            .json::<raw::Raw>()
            .await?
            .user
            .into();
        Ok(info)
    }
}
