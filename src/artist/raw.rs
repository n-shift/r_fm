use crate::{raw_gen, getter_usize};
use serde::Deserialize;
raw_gen!(pub Raw { pub artist: Artist });
raw_gen!(pub Stats {}, listeners, playcount);
raw_gen! {
    pub Artist {
        pub stats: Stats,
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
