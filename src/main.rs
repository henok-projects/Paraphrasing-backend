use actix_web::{post, web, App, HttpServer, Responder};
use actix_web::http::header;
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize)]
struct AIRequest {
    prompt: String,  // Changed from 'topic' to match your frontend
}

#[derive(Serialize, Deserialize)]
struct AIResponse {
    content: String,
}

#[post("/generate-poem")]
async fn generate_poem(request: web::Json<AIRequest>) -> Result<impl Responder, actix_web::Error> {
    let api_key = env::var("OPENAI_API_KEY").map_err(|_| {
        actix_web::error::ErrorInternalServerError("OPENAI_API_KEY not set")
    })?;

    let topic = format!("Write a short poem about {}", request.prompt);

    let client = reqwest::Client::new();
    let response = client
        .post("https://api.deepinfra.com/v1/openai/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "model": "meta-llama/Llama-2-70b-chat-hf",
            "messages": [{"role": "user", "content": topic}],
            "temperature": 0.7
        }))
        .send()
        .await
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("API request failed: {}", e))
        })?;

    let poem = response.json::<serde_json::Value>().await.map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Failed to parse response: {}", e))
    })?;

    let content = poem["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("Could not generate poem")
        .to_string();

    Ok(web::Json(AIResponse { content }))
}

#[post("/paraphrase")]
async fn paraphrase_text(request: web::Json<AIRequest>) -> Result<impl Responder, actix_web::Error> {
    let api_key = env::var("OPENAI_API_KEY").map_err(|_| {
        actix_web::error::ErrorInternalServerError("OPENAI_API_KEY not set")
    })?;

    let paraphrase_instruction = format!("Paraphrase the following text while keeping the same meaning: {}", request.prompt);

    let client = reqwest::Client::new();
    let response = client
        .post("https://api.deepinfra.com/v1/openai/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "model": "meta-llama/Llama-2-70b-chat-hf",
            "messages": [{"role": "user", "content": paraphrase_instruction}],
            "temperature": 0.7
        }))
        .send()
        .await
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("API request failed: {}", e))
        })?;

    let result = response.json::<serde_json::Value>().await.map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Failed to parse response: {}", e))
    })?;

    let content = result["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("Could not paraphrase text")
        .to_string();

    Ok(web::Json(AIResponse { content }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let port = std::env::var("PORT").unwrap_or_else(|_| "1000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    println!("Starting server at http://{}", addr);

    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("https://paraphrasing-frontend.vercel.app")
            .allowed_origin("http://localhost:3000")
            .allowed_headers(vec![
                        header::AUTHORIZATION,
                        header::ACCEPT,
                        header::CONTENT_TYPE,
                    ])
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(paraphrase_text)
            .service(generate_poem)
    })
    .bind(addr)?
    .run()
    .await
}

