use serde::Deserialize;

#[derive(Deserialize)]
pub struct QueryRequest {
    pub query: String,
}
