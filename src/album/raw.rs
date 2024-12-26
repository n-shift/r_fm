use crate::shared::SizedImages;
use crate::{getter_usize, raw_gen};
use serde::Deserialize;
// TODO: might want to reuse stats thing from artist::raw
raw_gen!(pub Raw { pub album: Album });
raw_gen! {
    pub Album {
        pub userplaycount: Option<usize>,
        pub mbid: Option<String>,
        pub image: Vec<SizedImages>,
    },
    artist,
    name,
    url,
    listeners,
    playcount,
}

getter_usize! {
    Album,
    pub AUsize,
    listeners = listeners,
    playcount = playcount,
}
