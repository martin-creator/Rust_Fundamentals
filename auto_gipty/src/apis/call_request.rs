use crate::models::general::llm::{ Message, ChatCompletion, APIResponse };
use dotenv::dotenv;
use reqwest::{ Client, header::{ HeaderMap, self, HeaderValue } };
use std::env;

// Call Large language model(.ie GTP-4)
pub async fn call_gpt(messages: Vec<Message>) -> Result<String, Box<dyn std::error::Error + Send>> {
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
    headers.insert("authorization", HeaderValue::from_str(&format!("Bearer {}", api_key))
    .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?);

    // Create Open AI Org header
    headers.insert("OpenAI-Organization", HeaderValue::from_str(api_org.as_str())
    .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?);

    // Create client
    let client = Client::builder().default_headers(headers).build()
    .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    // Create chat completion
    let chat_completion: ChatCompletion = ChatCompletion {
        model: "gpt-3.5-turbo-0613".to_string(),
        messages,
        temperature: 0.1,
        choices: 1,
    };

    // // Troubleshooting
    // let res_raw = client
    //   .post(url)
    //   .json(&chat_completion)
    //   .send()
    //   .await
    //   .unwrap();
    // dbg!(res_raw.text().await.unwrap());

    // Get API response
    let res: APIResponse = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?
        .json()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    // Send response
    Ok(res.choices[0].message.content.clone())

}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_call_to_openai() {
        let message = Message {
            role: "user".to_string(),
            content: "Hi there, I'm looking for a webserver. Give me some ideas".to_string(),
        };

        let messages: Vec<Message> = vec![message];
        let res = call_gpt(messages).await;

        match res {
        Ok(res_str) => {
            dbg!(res_str);
            assert!(true);
        },
        Err(e) => {
            dbg!(e);
            assert!(false);
        }
    }
}
}
