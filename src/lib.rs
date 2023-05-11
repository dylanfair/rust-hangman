
use reqwest::{Response, Error};


pub async fn make_word_request(request_url: &str) -> Result<Response, Error> {
    let response = reqwest::get(request_url).await?;
    response
}