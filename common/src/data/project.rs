use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Project {
    pub name: String,
    pub link: String,
    pub image: String,
}
