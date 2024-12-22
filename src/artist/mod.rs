mod raw;
use raw::SUsize;
use crate::from_raw;

#[allow(dead_code)]
#[derive(Debug)]
pub struct ArtistInfo {
    name: String,
    listeners: usize,
    playcount: usize,
}

impl From<raw::Artist> for ArtistInfo {
    fn from(item: raw::Artist) -> Self {
        from_raw! {
            item,
            { name },
            {
                listeners = item.stats.listeners(),
                playcount = item.stats.playcount()
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
    pub async fn get_info(&self, client: &Client) -> anyhow::Result<ArtistInfo> {
        ArtistInfo::get(client, self.0, self.1).await
    }
}

// TODO: lang, autocorrect, username
use super::Client;
impl ArtistInfo {
    async fn get(client: &Client, spec: Spec, val: &str) -> anyhow::Result<Self> {
        let params = &[("method", "artist.getInfo"), (spec.to_param(), val)];
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
