use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseObject {
    pub objects: Vec<SearchResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub package: Package,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub links: Link,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Link {
    pub npm: String,
    pub repository: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SingleResponseObject {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub repository: Option<Repo>,
    pub license: Option<String>,
    pub homepage: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Repo {
    pub url: Option<String>,
}
