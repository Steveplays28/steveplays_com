use serde::{Deserialize, Serialize};

use crate::data::author::Author;

#[derive(Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Art {
    pub name: String,
    pub authors: Vec<Author>,
    pub contributors: Option<Vec<Author>>,
    pub date: String,
    pub image: String,
    pub tags: Option<Vec<String>>,
}

impl PartialOrd for Art {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Art {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}
