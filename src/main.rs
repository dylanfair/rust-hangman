fn main() {
    let RANDOM_WORD_URL = "https://random-word-api.vercel.app/api?words=1";
    
    let response = rust_hangman::make_word_request(RANDOM_WORD_URL);

    println!("Hello, world!");
}
