use std::io;
use std::fs;
use rand::Rng;
use std::io::BufRead;
//updates the word shown to the user revealing all characters in the guesses vector
fn new_show_word(secret: &str, guesses: &Vec<char>) -> String {
    let mut result = String::with_capacity(secret.len());
    let secret: Vec<char> = secret.chars().collect();
    for i in 0..secret.len() {
        let to_add = match secret[i] {
            v if guesses.contains(&v) => secret[i],
            _ => '_'
        };
        result.push(to_add);
    }
    result
}
//gets guess input from user
fn get_guess() -> Option<char> {
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Reading input failed.");
    let safe_guess = guess.trim().chars().next(); //takes the first character of the input
    return safe_guess;
}
//loads words from the given file
fn load_from_file(filename: &str) -> io::Result<io::BufReader<fs::File>> {
    let file = fs::File::open(filename)?;
    let reader = io::BufReader::new(file);
    return Ok(reader);
}
//counts the words in a file (each on new line)
fn count_words() -> usize {
    match load_from_file("words.txt") {
        Err(_) => {
            println!("Error loading word file.");
            return 0;
        }
        Ok(result) => {
            return result.lines().count();
        }
    }
}
//chooses a random word
fn random_word() -> String {
    match load_from_file("words.txt") {
        Err(_) => {
            println!("Error loading word file. Starting game with the default word.");
            return String::from("hazmat");
        }
        Ok(result) => {
            let mut lines = result.lines();
            let idx = rand::thread_rng().gen_range(0, count_words());
            match lines.nth(idx) {
                None | Some(Err(_)) => {
                    println!("Error loading word file. Starting game with the default word.");
                    return String::from("hazmat");
                },
                Some(Ok(result)) => return String::from(result)
            };
        }
    }
}

fn main() {
    let word = random_word();
    let mut guesses: Vec<char> = vec![];
    let mut show = new_show_word(&word, &guesses); //the string that is shown to the player
    let mut turn_count = 7;

    loop {
        if turn_count == 0 {
            println!("You lost! The word was {}", word);
            break;
        }
        if !show.contains('_') {
            println!("Gongratulations! You won!");
            break;
        }

        let padded_string: Vec<&str> = show.split("").collect();
        println!("{}", padded_string.join(" ")); //add spaces between the characters
        println!("Guess a character:");

        match get_guess() {
            None => { //given invalid input
                turn_count -= 1;
                print!("Incorrect character given! ");
            },
            Some(guess) => { //given valid input
                if !guesses.contains(&guess) {
                    if word.contains(guess) {
                        print!("Correct! ");
                        guesses.push(guess);
                        show = new_show_word(&word, &guesses);
                    } else {
                        turn_count -= 1;
                        print!("Wrong! ");
                    }
                } else {
                    print!("You have already guessed that! ");
                }
            }
        };
        println!("You have {} turns left.", turn_count);
    }
}
