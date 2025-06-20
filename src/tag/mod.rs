mod raw;
use crate::from_raw;

#[derive(Debug)]
pub struct TagInfo {
    pub name: String,
    pub reach: usize,
    pub total: usize,
    pub wiki: raw::Wiki,
}

impl From<raw::Tag> for TagInfo {
    fn from(item: raw::Tag) -> Self {
        from_raw! {
            item,
            {
                name,
                wiki,
                reach,
                total
            },
            {}
        }
    }
}

use std::collections::HashMap;
pub struct Tag {
    name: String,
    params: HashMap<String, String>,
}

use super::Client;
use reqwest::Method;
impl Tag {
    pub fn new(name: String) -> Self {
        Self {
            name,
            params: HashMap::new(),
        }
    }
    pub async fn get_info(&self, client: &Client) -> anyhow::Result<TagInfo> {
        let r = client
            .build(Method::GET, "tag.getInfo")
            .query(&[("tag", self.name.as_str())])
            .query(&self.params);
        let i: TagInfo = r.send().await?.json::<raw::Raw>().await?.tag.into();
        Ok(i)
    }
}
