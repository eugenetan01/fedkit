use crate::controllers::llm_controller;
use crate::models::query_request::QueryRequest;
use actix_web::{post, web, HttpResponse, Responder};

#[post("/query")]
pub async fn handle_query(req: web::Json<QueryRequest>) -> impl Responder {
    let query = req.query.clone();
    match llm_controller::handle_llm_query(query).await {
        Ok(response) => {
            // Return Ollama's response in the HTTP response
            HttpResponse::Ok().json(response)
        }
        Err(err) => {
            // Handle errors and return an Internal Server Error
            HttpResponse::InternalServerError().body(format!("Error: {}", err))
        }
    }
    // HttpResponse::Ok().body(format!("Received query: {}", req.query))
}

// use crate::model::{query_request::QueryRequest, query_response::QueryResponse};
// use actix_web::{post, web, HttpResponse, Responder};
// use serde::{Deserialize, Serialize};

// #[derive(Deserialize)]
// pub struct QueryRequest {
//     query: String,
// }

// #[derive(Serialize)]
// pub struct QueryResponse {
//     //combined_result: serde_json::Value,
//     //individual_responses: serde_json::Value,
//     queries_sent: String,
// }

// #[post("/query")]
// async fn handle_query(req: web::Json<QueryRequest>) -> impl Responder {
//     let query = &req.query;

//     // Generate database-specific queries using LLM
//     //let queries = llm::process_query(query).await;

//     // Execute queries concurrently
//     //let postgres_result = db::postgres::execute_query(&queries["postgres"]).await;
//     //let mongodb_result = db::mongodb::execute_query(&queries["mongodb"]).await;

//     // Consolidate results
//     //let combined_result = json!({
//     //    "postgres": postgres_result,
//     //    "mongodb": mongodb_result,
//     //    "s3": s3_result
//     //});

//     HttpResponse::Ok().json(QueryResponse {
//         //combined_result,
//         //individual_responses: combined_result.clone(),
//         queries_sent: query.to_string(), //queries,
//     })
// }
