use rand::random;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs;
use std::io;

fn main() {
   //Read the contents in the text file
  let content = fs::read_to_string("words.txt").expect("Failed to read file");

  //Split the file into lines and store each line as a word
  let words: Vec<&str> = content.lines().collect();

  //random number generator
  let mut rng = thread_rng();
  //Pick a random word
  let random_word = words.choose(&mut rng).unwrap();

  //Starting the program
  println!("Welcome to Hangman!");
  starting_figure();

  struct Hangman {
    word: String,
    display: Vec<char>,
    wrong_guesses: u32,
  }

  let mut game = Hangman {
    word: random_word.to_string(),
    display: vec!['_'; random_word.chars().count()],
    wrong_guesses: 0,
  };


  //Create a display vector filled with underscores
  let mut display = vec!['_'; random_word.chars().count()];

  //Loop through the hangman program
  loop {
    //Print current progress in the word
    for letter in &game.display {
        print!("{} ", letter);
    }
    println!();

    //check if the word is fully guessed
    if !display.contains(&'_') {
        println!("You guessed the word!");
        break;
    }
    
    //Ask for a letter
    println!("Guess a letter: ");
    //store the users input
    let mut input = String::new();
    //Read the player's input
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    //Convert input into a single character
    let guess = match input.trim().chars().next() {
        Some(c) => c,
        None => {
            println!("Please type a letter! ");
            continue;;
        }
    };

    //Track whether the guessed letter is in the word
    let mut found = false;
    //Loop through each letter of the random word
    for (i, l) in game.word.chars().enumerate() {
        //If the guessed letter matches a lettter in the word
        if l == guess {
            //REveal the letter in the display vector
            game.display[i] = l;
            //Mark that the guess was correct
            found = true;
        }
    }

    
    //If the letter wasn't found, increase the wrong guess counter
    if !found {
        println!("Letter '{}' is not in the word.", guess);
        game.wrong_guesses += 1;
    }

    //Draw the hangman depending on how many wrong guesses
    if game.wrong_guesses == 1 {
        one_wrong();
    } else if game.wrong_guesses == 2{
        two_wrong();
    } else if game.wrong_guesses == 3{
        three_wrong();
    } else if game.wrong_guesses == 4{
        four_wrong();
    } else if game.wrong_guesses == 5 {
        five_wrong();
    } else if game.wrong_guesses == 6 {
        six_wrong();
    } else if game.wrong_guesses == 7 {
        println!("You didn't guess the word!");
        println!("The word was {}", random_word);
        seven_wrong();
        break;
    }
}

}
//the function that has the starting figure
fn starting_figure() {
    println!("  _____________ ");
    println!(" |             |");
    println!("               |");
    println!("               |");
    println!("               |");
    println!("               |");
    println!("               |");
    println!("               |");
    println!("               |");
    println!("               |");
    println!("               |");
    println!("               |");
    println!("^^^^^^^^^^^^^^^ ");
}
//the function that has the head added
fn one_wrong() {
    println!("  _____________ ");
    println!(" |             |");
    println!(" ___           |");
    println!("|   |          |");
    println!(" ---           |");
    println!("               |");
    println!("               |");
    println!("               |");
    println!("               |");
    println!("               |");
    println!("               |");
    println!("               |");
    println!("^^^^^^^^^^^^^^^ ");
}
//the function that has the body added
fn two_wrong() {
    println!("  _____________ ");
    println!(" |             |");
    println!(" ___           |");
    println!("|   |          |");
    println!(" ---           |");
    println!("  |            |");
    println!("  |            |");
    println!("  |            |");
    println!("  |            |");
    println!("  |            |");
    println!("  |            |");
    println!("               |");
    println!("^^^^^^^^^^^^^^^ ");
}
//the function that has a leg added
fn three_wrong() {
    println!("  _____________ ");
    println!(" |             |");
    println!(" ___           |");
    println!("|   |          |");
    println!(" ---           |");
    println!("  |            |");
    println!("  |            |");
    println!("  |            |");
    println!("  |            |");
    println!("  |            |");
    println!("  |            |");
    println!(" /             |");
    println!("^^^^^^^^^^^^^^^ ");
}
//the function that has a arm added
fn four_wrong() {
    println!("  _____________ ");
    println!(" |             |");
    println!(" ___           |");
    println!("|   |          |");
    println!(" ---           |");
    println!("  |            |");
    println!("  | /          |");
    println!("  |/           |");
    println!("  |            |");
    println!("  |            |");
    println!("  |            |");
    println!(" /             |");
    println!("^^^^^^^^^^^^^^^ ");
}
//the function that has another leg added
fn five_wrong() {
    println!("  _____________ ");
    println!(" |             |");
    println!(" ___           |");
    println!("|   |          |");
    println!(" ---           |");
    println!("  |            |");
    println!("  | /          |");
    println!("  |/           |");
    println!("  |            |");
    println!("  |            |");
    println!("  |            |");
    println!(" / \\           |");
    println!("^^^^^^^^^^^^^^^ ");
}
//the function that has another arm added
fn six_wrong() {
    println!("  _____________ ");
    println!(" |             |");
    println!(" ___           |");
    println!("|   |          |");
    println!(" ---           |");
    println!("  |            |");
    println!("\\ | /          |");
    println!(" \\|/           |");
    println!("  |            |");
    println!("  |            |");
    println!("  |            |");
    println!(" / \\           |");
    println!("^^^^^^^^^^^^^^^ ");
}
//the function that shows the dead man
fn seven_wrong() {
    println!("  _____________ ");
    println!(" |             |");
    println!(" ____          |");
    println!("|x x |         |");
    println!(" ----          |");
    println!("  |            |");
    println!("\\ | /          |");
    println!(" \\|/           |");
    println!("  |            |");
    println!("  |            |");
    println!("  |            |");
    println!(" / \\           |");
    println!("^^^^^^^^^^^^^^^ ");
}