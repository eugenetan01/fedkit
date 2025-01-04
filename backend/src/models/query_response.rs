use serde::Serialize;

#[derive(Serialize)]
pub struct QueryResponse {
    pub queries_sent: String,
}
