use crate::models::general::llm::{ Message, ChatCompletion };
use dotenv::dotenv;
use reqwest::{ Client, header::{ HeaderMap, self, HeaderValue } };
use std::env;

// Call Large language model(.ie GTP-4)
pub async fn call_gpt(messages: Vec<Message>) {
    dotenv().ok();

    // Extract API Key Information
    let api_key: String = env
        ::var("OPEN_AI_KEY")
        .expect("OPEN_AI_KEY not found in your environment");
    let api_org: String = env
        ::var("OPEN_AI_ORG")
        .expect("OPEN_AI_ORG not found in your environment");

    // confirm endpoint
    let url: &str = "https://api.openai.com/v1/chat/completions";

    // Create headers
    let mut headers: HeaderMap = HeaderMap::new();

    // Create api key header
    headers.insert("authorization", HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap());

    // Create Open AI Org header
    headers.insert("OpenAI-Organization", HeaderValue::from_str(api_org.as_str()).unwrap());

    // Create client
    let client = Client::builder().default_headers(headers).build().unwrap();

    // Create chat completion
    let chat_completion: ChatCompletion = ChatCompletion {
        model: "gpt-3.5-turbo-0613".to_string(),
        messages,
        temperature:0.1,
    };

     // // Troubleshooting
    let res_raw = client
      .post(url)
      .json(&chat_completion)
      .send()
      .await
      .unwrap();
    dbg!(res_raw.text().await.unwrap());
    
}


