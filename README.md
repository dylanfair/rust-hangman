# rust-hangman

(not part of the rust foundation btw)

My curious attempt at creating a hangman game in Rust. I'm sure this could have been done way better, but happy to get it working.

## Areas of improvement

### print_hangman

An obvious one is the `print_hangman` function which is just straight up hardcoded. Ideally some way of appending the proper stick figure parts as the `tries` go down would be best, but wasn't quite sure what the best approach would be.

### Difficulty Settings

Currently the game has the user guess for an 8 letter word with 10 incorrect guesses allowed. I think it'd be cool if the user could adjust both the length of the word and the amount of guesses allowed themselves, but ~I'm currently relying on length of the word being constant to implement the current game~ Update: looks like I was just making my vectors wrong, adjusting the vector size is super doable now. I'd also have to rethink how to print the hangman guy if amount of tries were to be a constantly shifting number.
