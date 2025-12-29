use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Author {
    pub name: String,
    pub website: Option<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Images {
    pub icon: Option<String>,
    pub banner: Option<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Links {
    pub website: Option<String>,
    pub repository: Option<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Project {
    pub name: String,
    pub authors: Option<Vec<Author>>,
    pub contributors: Option<Vec<Author>>,
    pub images: Option<Images>,
    pub links: Option<Links>,
    pub tags: Option<Vec<String>>,
}
