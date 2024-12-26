mod raw;
use crate::from_raw;
use crate::shared::SizedImages;
use raw::Bio;
use raw::SUsize;

#[allow(dead_code)]
#[derive(Debug)]
pub struct ArtistInfo {
    name: String,
    mbid: Option<String>,
    listeners: usize,
    playcount: usize,
    userplaycount: Option<usize>,
    image: Vec<SizedImages>,
    url: String,
    bio: Bio,
}

impl From<raw::Artist> for ArtistInfo {
    fn from(item: raw::Artist) -> Self {
        from_raw! {
            item,
            {
                name,
                mbid,
                image,
                url,
                bio
            },
            {
                listeners = item.stats.listeners(),
                playcount = item.stats.playcount(),
                userplaycount = item.stats.userplaycount()
            }
        }
    }
}

impl From<raw::SArtist> for SimilarArtist {
    fn from(item: raw::SArtist) -> Self {
        from_raw! {
            item,
            {
                name,
                mbid
            },
            {
                similarity = item.similarity()
            }
        }
    }
}

#[derive(Clone, Copy)]
pub enum Spec {
    Name,
    Mbid,
}

impl Spec {
    fn as_str(&self) -> &str {
        match self {
            Self::Name => "artist",
            Self::Mbid => "mbid",
        }
    }
}

use std::collections::HashMap;
pub struct Artist {
    spec: Spec,
    id: String,
    pub params: HashMap<String, String>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct SimilarArtist {
    name: String,
    mbid: Option<String>,
    similarity: f32,
}

use reqwest::Method;
impl Artist {
    pub async fn get_info(&self, client: &Client) -> anyhow::Result<ArtistInfo> {
        let r = client
            .build(Method::GET)
            .query(&[
                ("method", "artist.getInfo"),
                (self.spec.as_str(), self.id.as_str()),
            ])
            .query(&self.params);
        let i: ArtistInfo = r.send().await?.json::<raw::Raw>().await?.artist.into();
        Ok(i)
    }
    pub async fn get_similar(&self, client: &Client) -> anyhow::Result<Vec<SimilarArtist>> {
        let r = client
            .build(Method::GET)
            .query(&[
                ("method", "artist.getSimilar"),
                (self.spec.as_str(), self.id.as_str()),
            ])
            .query(&self.params);
        let s = r
            .send()
            .await?
            .json::<raw::SimilarArtists>()
            .await?
            .similarartists
            .artist
            .into_iter()
            .map(SimilarArtist::from)
            .collect::<Vec<_>>();
        Ok(s)
    }
    pub async fn search(&self, client: &Client) -> anyhow::Result<Vec<String>> {
        let r = client
            .build(Method::GET)
            .query(
                &[
                    ("method", "artist.search"),
                    (self.spec.as_str(), self.id.as_str()),
                ])
            .query(&self.params);
        let l = r
            .send()
            .await?
            .json::<raw::MatchedArtists>()
            .await?
            .results
            .artistmatches
            .artist
            .into_iter()
            .map(|m| m.name)
            .collect::<Vec<_>>();
        Ok(l)
    }
    pub fn new(spec: Spec, id: String) -> Self {
        Self {
            spec,
            id,
            params: HashMap::new(),
        }
    }
}

use super::Client;
