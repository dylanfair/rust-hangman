#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    println!("Please choose how many letters you'd like your word to have (between 6-10):");
    let word_length = rust_hangman::determine_word_length();
    let mut tries = 10;

    let word = rust_hangman::make_word_request(word_length).await?;
    
    // so now we have our word
    // next would be making x spaces depending on length of word
    let mut display_vector = vec!["_".to_string(); word_length];

    let mut incorrect_guesses = vec![]; // place the incorrect letters a user guesses can go
    
    // loop here
    let mut print_statement = "Welcome to hangman!";
    loop {
        clearscreen::clear().expect("failed to clear the screen");
        
        // println!("Word is: {}", word);
        println!("{}", display_vector.join(" "));
        println!("{}", print_statement);
        println!("Incorrect guesses left: {tries}");
        println!("Prior guesses: {:?}", incorrect_guesses);
        rust_hangman::print_hangman(tries);

        println!("Make your guess: ");
        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess).unwrap();

        // Clean and validate guess
        let guess = guess.replace("\n", "");        
        if guess.len() > 1 || guess.len() == 0 || !guess.chars().all(char::is_alphabetic) {
            print_statement = "Your guess must be a single letter.";
            continue;
        }

        // See if that letter was guessed already
        if display_vector.contains(&guess) || incorrect_guesses.contains(&guess) {
            print_statement = "You already guessed that letter, try again.";
            continue;
        }

        // Logic if user guesses correctly!
        if word.contains(&guess) {
            for (i, char) in word.chars().enumerate() {
                if &char.to_string() == &guess {
                    display_vector[i] = guess.clone();
                }
            }
            print_statement = "You got it!";
            
            if !display_vector.contains(&String::from("_")) {
                clearscreen::clear().expect("failed to clear the screen");
                println!("{}", display_vector.join(" "));
                println!("Congratulations!! You won the game!");
                break;
            }
            continue;
        }

        // If not correct, then we push and move tries down one
        incorrect_guesses.push(guess);
        print_statement = "Incorrect :(";
        // Lose condition
        tries = tries - 1;
        if tries == 0  {
            clearscreen::clear().expect("This should work");
            println!("{}", display_vector.join(" "));
            println!("Prior guesses: {:?}", incorrect_guesses);
            rust_hangman::print_hangman(tries);
            println!("You lost! The word was '{}'. Feel free to try again!", word);
            break;
        }
    }

    Ok(())
}
