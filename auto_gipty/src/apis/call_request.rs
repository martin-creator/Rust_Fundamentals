use crate::models::general::llm::{Message};
use dotenv::dotenv;
use reqwest::{Client, header::{HeaderMap, self, HeaderValue}};
use std::env;

// Call Large language model(.ie GTP-4)
pub async fn call_gpt(messages: Vec<Message>) {
    dotenv().ok();

    // Extract API Key Information
    let api_key: String = env::var("OPEN_AI_KEY").expect("OPEN_AI_KEY not found in your environment");
    let api_org: String = env::var("OPEN_AI_ORG").expect("OPEN_AI_ORG not found in your environment");

    // confirm endpoint
    let url :&str = "https://api.openai.com/v1/chat/completions";

    // Create headers
    let mut headers: HeaderMap = HeaderMap::new();

    // Create api key header
    headers.insert(
        "authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap()
    );

    // Create Open AI Org header
    headers.insert(
        "OpenAI-Organization",
        HeaderValue::from_str(api_org.as_str()).unwrap()
    );

    // Create client
    let client = Client::builder()
    .default_headers(headers)
    .build()
    .unwrap();
}