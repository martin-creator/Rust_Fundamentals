use std::io::{Error, ErrorKind};

async fn my_async_call(url:&str) -> Result<serde_json::Value, Error> {
    
    let response: reqwest::Response = reqwest::get(url)
    .await
    .map_err(|_| Error::new(ErrorKind::Other, "Failed to get response"))?;

    let json_response = response
    .json::<serde_json::Value>()
    .await
    .map_err(|_| Error::new(ErrorKind::Other, "Could not decode to JSON"))?;
    
    Ok(json_response)
}


#[cfg(test)]

mod tests {

    use super::*;

    #[tokio::test]

    async fn tests_calls_async(){
        let api_url:&str = "https://opentdb.com/api.php?amount=29";
        let my_res: Result<serde_json::Value, std::io::Error >= my_async_call(api_url).await;
        match my_res {
            Ok(res) => {
                println!("Response: {:?}", res);
                assert_eq!(res["response_code"], 0);
            },
            Err(e) => {
                println!("Error: {:?}", e);
                assert!(false);
            }
        };

    }
}