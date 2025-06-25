use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guessing Game!");

    let secret_word = rand::rng().random_range(1..=30);

    const GUESSING_NUMBER_LIMIT: u8 = 30;

    //println!("secret_word : {}", secret_word);
    loop {
        println!("Enter your guess number : ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut  guess)
            .expect("Can't read the line");
        println!("your guessed number is {}",guess);

        let guess: u8 = match guess.trim().parse() {
            Ok(val) => {
                if val > GUESSING_NUMBER_LIMIT {
                    println!("the range is from 1 to {}", GUESSING_NUMBER_LIMIT);
                    continue;
                } else {
                    val
                }
            }
            Err(_) => {
                println!("Please enter a valid number between 1 to 30");
                continue;
            }
        };

        match guess.cmp(&secret_word) {
            Ordering::Less => println!("small"),
            Ordering::Greater => println!("greater"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        } //end of match
    } //end of loop
}

/*
 * a;lsdkjf
 * a;lsdkjfasl;lsdkjf
 *
 *
 * al;lsdkjf
 *
 *
 * asdlfkj aa asdlfkj asdlfkj
 * end
 *
 * */
