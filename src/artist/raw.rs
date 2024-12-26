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

raw_gen!(pub SimilarArtists {pub similarartists: ArtistList});
raw_gen!(pub ArtistList { pub artist: Vec<SArtist>});
raw_gen!(pub SArtist { pub mbid: Option<String>,}, name, r#match);

raw_gen!(pub MatchedArtists { pub results: Results });
raw_gen!(pub Results { pub artistmatches: ArtistMatches });
raw_gen!(pub ArtistMatches { pub artist: Vec<Match> });
raw_gen!(pub Match { pub name: String });

getter_usize! {
    Stats,
    pub SUsize,
    listeners = listeners,
    playcount = playcount,
}

impl Stats {
    pub fn userplaycount(&self) -> Option<usize> {
        let x = self.userplaycount.clone();
        x.as_ref()?;
        Some(x.unwrap().parse::<usize>().unwrap())
    }
}

impl SArtist {
    pub fn similarity(&self) -> f32 {
        self.r#match.parse::<f32>().unwrap()
    }
}
