mod raw;
use crate::from_raw;
use crate::shared::SizedImages;
use raw::AUsize;

#[derive(Debug)]
pub struct AlbumInfo {
    pub name: String,
    pub artist: String,
    pub mbid: Option<String>,
    pub image: Vec<SizedImages>,
    pub listeners: usize,
    pub playcount: usize,
    pub userplaycount: Option<usize>,
    pub url: String,
}

impl From<raw::Album> for AlbumInfo {
    fn from(item: raw::Album) -> Self {
        from_raw! {
            item,
            {
                name,
                artist,
                mbid,
                image,
                url,
                userplaycount
            },
            {
                listeners = item.listeners(),
                playcount = item.playcount()
            }
        }
    }
}

use std::collections::HashMap;
#[derive(Debug)]
pub struct Album {
    spec: Spec,
    pub params: HashMap<String, String>,
}

#[derive(Debug)]
pub enum Spec {
    Explicit(String, String),
    Mbid(String),
}

use super::Client;
use reqwest::Method;
impl Album {
    pub fn new(spec: Spec) -> Self {
        Self {
            spec,
            params: HashMap::new(),
        }
    }
    pub async fn get_info(&self, client: &Client) -> anyhow::Result<AlbumInfo> {
        let r = client
            .build(Method::GET)
            .query(&[("method", "album.getInfo")])
            .query(
                match &self.spec {
                    Spec::Explicit(artist, album) => {
                        vec![("artist".to_owned(), artist), ("album".to_owned(), album)]
                    }
                    Spec::Mbid(mbid) => vec![("mbid".to_owned(), mbid)],
                }
                .as_slice(),
            )
            .query(&self.params);
        let i: AlbumInfo = r.send().await?.json::<raw::Raw>().await?.album.into();
        Ok(i)
    }
}
