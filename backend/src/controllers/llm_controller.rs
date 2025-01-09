use crate::models::{ollama_request::OllamaRequest, ollama_response::OllamaResponse};
use reqwest::Client;

pub async fn handle_llm_query(query: String) -> Result<OllamaResponse, reqwest::Error> {
    let client = Client::new();
    let url = "http://localhost:11434/api/generate"; // Replace with your Ollama API endpoint.

    let request = OllamaRequest {
        model: "llama3".to_string(),
        prompt: query,
        stream: false,
    };

    let response = client
        .post(url)
        .json(&request)
        .send()
        .await?
        .json::<OllamaResponse>()
        .await?;

    println!("Ollama Response: {}", response.response);

    Ok(response)
}
