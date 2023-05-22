use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseObj {
    pub crates: Vec<Crate>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Crate {
    name: String,
    id: String,
    description: String,
    downloads: i32,
    exact_match: bool,
    documentation: String,
    newest_version: String,
}
