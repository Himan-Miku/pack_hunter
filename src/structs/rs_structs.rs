use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseObj {
    pub crates: Vec<Crate>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Crate {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub documentation: Option<String>,
    pub repository: Option<String>,
    pub downloads: u64,
    pub max_stable_version: Option<String>,
    pub exact_match: Option<bool>,
    pub max_version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SingleResponseObj {
    pub version: Version,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    #[serde(rename = "crate")]
    pub crate_name: String,
    pub downloads: u64,
    pub id: u64,
    pub license: String,
    pub readme_path: String,
}
