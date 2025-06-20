use crate::{getter_usize, raw_gen};
use serde::Deserialize;

raw_gen!(pub Raw { pub track: Track });
raw_gen! {
    pub Track {
        pub userplaycount: Option<String>,
        pub userloved: Option<String>,
        pub artist: RawArtist,
        pub album: Option<RawAlbum>,
    },
    name,
    url,
    duration,
    listeners,
    playcount,
    artist: RawArt,
    album: Option<RawAlb>,
}
raw_gen!(pub RawArtist {}, name);
raw_gen!(pub RawAlbum {}, artist, title);

getter_usize! {
    Track,
    pub TUsize,
    listeners = listeners,
    playcount = playcount,
    duration = inter_d,
}

impl Track {
    pub fn duration(&self) -> Option<usize> {
        let i = self.inter_d();
        if i == 0 { None } else { Some(i) }
    }
    // from
    pub fn artist() -> crate::artist::Artist {
        crate::artist::Artist::from(crate::artist::Spec::)
    };
    // from
    pub fn album();
    pub fn userplaycount();
    pub fn userloved();
}
