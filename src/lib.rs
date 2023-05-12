/// Makes an API call to get a random word
pub async fn make_word_request(word_length: usize) -> Result<String, reqwest::Error> {
    let random_word_url = format!("https://random-word-api.herokuapp.com/word?length={word_length}");
    let res = reqwest::get(random_word_url).await?;

    let body = res.text().await?;
    
    let word = String::from(body).replace(&['[', ']', '"'][..], "");
    Ok(word)
}