use serde::{Deserialize, Serialize};

use crate::data::author::Author;

#[derive(Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Images {
    pub icon: Option<String>,
    pub banner: Option<String>,
}

#[derive(Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Links {
    pub website: Option<String>,
    pub repository: Option<String>,
}

#[derive(Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Project {
    pub name: String,
    pub authors: Option<Vec<Author>>,
    pub contributors: Option<Vec<Author>>,
    pub images: Option<Images>,
    pub links: Option<Links>,
    pub tags: Option<Vec<String>>,
}

impl PartialOrd for Project {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Project {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}
