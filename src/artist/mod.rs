mod raw;
use crate::from_raw;
use crate::shared::{SizedImages, Opts};
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

pub struct Artist<'a>(pub Spec, pub &'a str);
impl Artist<'_> {
    pub async fn get_info(&self, client: &Client, opts: Opts<'_>) -> anyhow::Result<ArtistInfo> {
        ArtistInfo::get(client, self.0, self.1, opts).await
    }
}

use super::Client;
impl ArtistInfo {
    async fn get(client: &Client, spec: Spec, val: &str, opts: Opts<'_>) -> anyhow::Result<Self> {
        let dpar = &[("method", "artist.getInfo"), (spec.to_param(), val)];
        let params = &[dpar, opts.unwrap_or(&[])].concat();
        let info: ArtistInfo = client
            .build(params)
            .send()
            .await?
            .json::<raw::Raw>()
            .await?
            .artist
            .into();
        Ok(info)
    }
}
