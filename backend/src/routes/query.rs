use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct QueryRequest {
    query: String,
}

#[derive(Serialize)]
pub struct QueryResponse {
    //combined_result: serde_json::Value,
    //individual_responses: serde_json::Value,
    queries_sent: String,
}

#[post("/query")]
async fn handle_query(req: web::Json<QueryRequest>) -> impl Responder {
    let query = &req.query;

    // Generate database-specific queries using LLM
    //let queries = llm::process_query(query).await;

    // Execute queries concurrently
    //let postgres_result = db::postgres::execute_query(&queries["postgres"]).await;
    //let mongodb_result = db::mongodb::execute_query(&queries["mongodb"]).await;

    // Consolidate results
    //let combined_result = json!({
    //    "postgres": postgres_result,
    //    "mongodb": mongodb_result,
    //    "s3": s3_result
    //});

    HttpResponse::Ok().json(QueryResponse {
        //combined_result,
        //individual_responses: combined_result.clone(),
        queries_sent: query.to_string(), //queries,
    })
}
