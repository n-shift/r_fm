use crate::shared::SizedImages;
use crate::{getter_usize, raw_gen};
use serde::Deserialize;
raw_gen!(pub Raw { pub artist: Artist });
raw_gen! {
    pub Stats {
        pub userplaycount: Option<String>,
    },
    listeners,
    playcount,
}
raw_gen! {
    pub Bio {},
    published,
    summary,
    content,
}
raw_gen! {
    pub Artist {
        pub stats: Stats,
        pub image: Vec<SizedImages>,
        pub mbid: Option<String>,
        pub bio: Bio,
    },
    name,
    url,
}

getter_usize! {
    Stats,
    pub SUsize,
    listeners = listeners,
    playcount = playcount,
}

impl Stats {
    pub fn userplaycount(&self) -> Option<usize> {
        let x = self.userplaycount.clone();
        if x.is_none() {
            return None;
        }
        Some(x.unwrap().parse::<usize>().unwrap())
    }
}
