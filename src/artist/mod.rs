mod raw;
use crate::from_raw;
use crate::shared::SizedImages;
use raw::Bio;
use raw::SUsize;

#[derive(Debug)]
pub struct ArtistInfo {
    pub name: String,
    pub mbid: Option<String>,
    pub listeners: usize,
    pub playcount: usize,
    pub userplaycount: Option<usize>,
    pub image: Vec<SizedImages>,
    pub url: String,
    pub bio: Bio,
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

#[derive(Debug)]
pub struct SimilarArtist {
    pub name: String,
    pub mbid: Option<String>,
    pub similarity: f32,
}

use reqwest::Method;
impl Artist {
    pub async fn get_info(&self, client: &Client) -> anyhow::Result<ArtistInfo> {
        let r = client
            .build(Method::GET, "artist.getInfo")
            .query(&[(self.spec.as_str(), self.id.as_str())])
            .query(&self.params);
        let i: ArtistInfo = r.send().await?.json::<raw::Raw>().await?.artist.into();
        Ok(i)
    }
    pub async fn get_similar(&self, client: &Client) -> anyhow::Result<Vec<SimilarArtist>> {
        let r = client
            .build(Method::GET, "artist.getSimilar")
            .query(&[(self.spec.as_str(), self.id.as_str())])
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
            .build(Method::GET, "artist.search")
            .query(&[(self.spec.as_str(), self.id.as_str())])
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
