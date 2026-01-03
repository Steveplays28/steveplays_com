use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Author {
    pub name: String,
    pub website: Option<String>,
}
