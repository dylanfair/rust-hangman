use indoc::indoc;

pub fn determine_word_length() -> usize {
    loop {
        let mut number_input = String::new();
        std::io::stdin().read_line(&mut number_input).unwrap();

        match number_input.replace("\n", "").parse::<usize>() {
            Ok(n) => {
                if n > 10 {
                    println!("Please choose a number less than or equal to 10");
                    continue
                }
                if n < 6 {
                    println!("Please choose a number greater or equal to than 6");
                    continue
                }
                return n
            }
            Err(e) => {
                println!("{}", e);
                continue
            }
        }
    }
}

/// Makes an API call to get a random word
pub async fn make_word_request(word_length: usize) -> Result<String, reqwest::Error> {
    loop {
        let random_word_url = format!("https://random-word.ryanrk.com/api/en/word/random/?length={word_length}");
        let res = reqwest::get(random_word_url).await?;
    
        let body = res.text().await?;
        
        let word = String::from(body).replace(&['[', ']', '"'][..], "");
        if word.chars().all(char::is_alphabetic) { // sometimes the API returns a word with a - or ' in it, if it does we just ask for another word
            return Ok(word.to_lowercase())
        }
    }
    
}

// Responsible for printing our hangman, it's ugly code I know
pub fn print_hangman(tries: usize) {
    let mut hangman = indoc! {"
    "};
    match tries {
        10 => {
            hangman = indoc! {"
                --------
                |      |
                |      
                |      
                |      
                |      
                |_____
            "};
        }
        9 => {
            hangman = indoc! {"
                --------
                |      |
                |      O
                |      
                |      
                |      
                |_____
            "};
        }
        8 => {
            hangman = indoc! {"
                --------
                |      |
                |      O
                |      |
                |      
                |      
                |_____
            "};
        }
        7 => {
            hangman = indoc! {"
                --------
                |      |
                |      O
                |      |
                |      |
                |      
                |_____
            "};
        }
        6 => {
            hangman = indoc! {"
                --------
                |      |
                |      O
                |      |/
                |      |
                |      
                |_____
            "};
        }
        5 => {
            hangman = indoc! {"
                --------
                |      |
                |      O
                |     \\|/
                |      |
                |      
                |_____
            "};
        }
        4 => {
            hangman = indoc! {"
                --------
                |      |
                |      O
                |     \\|/
                |      |
                |     / 
                |_____
            "};
        }
        3 => {
            hangman = indoc! {"
                --------
                |      |
                |      O
                |     \\|/
                |      |
                |     / \\
                |_____
            "};
        }
        2 => {
            hangman = indoc! {"
                --------
                |      |
                |      O
                |     \\|/
                |      |
                |    _/ \\
                |_____
            "};
        }
        1 => {
            hangman = indoc! {"
                --------
                |      |
                |      O
                |     \\|/
                |      |
                |    _/ \\_
                |_____
            "};
        }
        0 => {
            hangman = indoc! {"
                --------
                |      |
                |      O     I'm dead!!!!
                |     \\|/
                |      |
                |    _/ \\_
                |_____
            "};
        }
        _ => println!("Shouldn't ever get here")
    }
    println!("{}", hangman)
}