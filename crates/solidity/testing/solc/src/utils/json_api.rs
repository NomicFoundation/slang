use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize)]
pub struct ApiInput {
    pub language: String,
    pub sources: HashMap<String, InputSource>,
}

#[derive(Debug, Serialize)]
pub struct InputSource {
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct ApiOutput {
    pub errors: Option<Vec<Error>>,
}

#[derive(Debug, Deserialize)]
pub struct Error {
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct OutputSource {
    pub ast: Value,
}
