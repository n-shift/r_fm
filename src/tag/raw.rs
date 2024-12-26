use crate::raw_gen;
use serde::Deserialize;
raw_gen!(pub Raw { pub tag: Tag });
raw_gen! {
    pub Tag {
        pub wiki: Wiki,
        // surprise
        pub reach: usize,
        pub total: usize,
    },
    name,
}
raw_gen!(pub Wiki {}, summary, content);
