mod raw;
use crate::artist::Artist;
use crate::album::Album;

#[derive(Debug)]
pub struct TrackInfo {
    pub name: String,
    pub artist: Artist;
    pub album: Option<Album>,
    pub duration: Option<usize>,
    pub listeners: usize,
    pub playcount: usize,
    pub userplaycount: Option<usize>,
    pub userloved: Option<bool>,
    pub url: String,
}

impl From<raw::Track for TrackInfo {
    fn from(item: raw::Album) -> Self {
        from_raw! {
            item,
            {
                name,
                url
            },
            {
                artist = ,
                album = ,
                duration = ,
                listeners = ,
                playcount = ,
                userplaycount = ,
                userloved = 
            }
        }
    }
}

use std::collections::HashMap;
#[derive(Debug)]
pub struct Track {
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
impl Track {
    pub fn new(spec: Spec) -> Self {
        Self {
            spec,
            params: HashMap::new(),
        }
    }
    pub async fn get_info(&self, client: &Client) -> anyhow::Result<AlbumInfo> {
        let r = client
            .build(Method::GET, "track.getInfo")
            .query(
                match &self.spec {
                    Spec::Explicit(artist, track) => {
                        vec![("artist".to_owned(), artist), ("track".to_owned(), track)]
                    }
                    Spec::Mbid(mbid) => vec![("mbid".to_owned(), mbid)]
                }
                .as_slice()
            )
            .query(&self.params);
        let i: TrackInfo = r.send().await?.json::<raw::Raw>().await?.track.into();
        Ok(i)
    }
}
