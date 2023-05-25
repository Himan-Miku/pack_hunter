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
