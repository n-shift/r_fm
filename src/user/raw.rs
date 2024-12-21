use serde::Deserialize;
use crate::{getter_bool, getter_usize, raw_gen};
pub type SizedImages = Vec<std::collections::HashMap<String, String>>;

raw_gen!(pub Raw { pub user: User });
raw_gen!(pub Registered {}, unixtime);
raw_gen! {
    pub User {
        pub image: SizedImages,
        pub registered: Registered,
    }
    name,
    realname,
    age,
    country,
    gender,
    subscriber,
    bootstrap,
    playlists,
    playcount,
    artist_count,
    album_count,
    track_count,
    url,
}

getter_bool!(User, pub URBool, is_pro = subscriber);
getter_usize!(Registered, pub RegUsize, time = unixtime);
getter_usize! {
    User,
    pub URUsize,
    aged = age,
    bootstraps = bootstrap,
    lists = playlists,
    plays = playcount,
    artists = artist_count,
    albums = album_count,
    tracks = track_count,
}


