use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OllamaResponse {
    pub response: String,
}
