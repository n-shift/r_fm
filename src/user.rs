use serde::Deserialize;
use std::convert::From;

use crate::{from_raw, getter_usize, raw_gen};

type SizedImages = Vec<std::collections::HashMap<String, String>>;

raw_gen!(Registered {}, unixtime);
getter_usize!(Registered, RegGetters, time = unixtime);

raw_gen! {
    UserRaw {
        image: SizedImages,
        registered: Registered,
    }
    name,
    realname,
    age,
    country,
    gender,
    subscriber,
    bootstrap,
    playlists,
    playcount,
    artist_count,
    album_count,
    track_count,
    url,
}
raw_gen!(Raw { user: UserRaw });

getter_usize! {
    UserRaw,
    URGetters,
    aged = age,
    subscribers = subscriber,
    bootstraps = bootstrap,
    lists = playlists,
    plays = playcount,
    artists = artist_count,
    albums = album_count,
    tracks = track_count,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct UserInfo {
    name: String,
    realname: String,
    age: usize,
    country: String,
    gender: String,
    subscriber: usize,
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

impl From<UserRaw> for UserInfo {
    fn from(item: UserRaw) -> Self {
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
                subscriber = item.subscribers(),
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
            .json::<Raw>()
            .await?
            .user
            .into();
        Ok(info)
    }
}
