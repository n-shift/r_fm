mod raw;
use crate::from_raw;
use crate::shared::{SizedImages, Param};
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

#[derive(Clone, Copy)]
pub enum Spec {
    Name,
    Mbid,
}

impl Spec {
    fn to_param(&self) -> &str {
        match self {
            Self::Name => "artist",
            Self::Mbid => "mbid",
        }
    }
}

pub struct Artist {
    spec: Spec,
    id: String,
    params: Option<Vec<Param>>,
}

impl Artist {
    pub async fn get_info(&self, client: &Client) -> anyhow::Result<ArtistInfo> {
        let r = client
            .build(&[("method", "artist.getInfo")])
            .query(&[(self.spec.to_param(), self.id.as_str())])
            .query(&self.params.clone().unwrap_or_default());
        let i: ArtistInfo = r
            .send()
            .await?
            .json::<raw::Raw>()
            .await?
            .artist
            .into();
        Ok(i)
    }
    pub fn new(spec: Spec, id: String) -> Self {
        Self { spec, id, params: None }
    }
    pub fn params(self, params: Vec<Param>) -> Self {
        Self {
            spec: self.spec,
            id: self.id,
            params: Some(params),
        }
    }
}

use super::Client;
