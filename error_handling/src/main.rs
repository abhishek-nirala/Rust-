use std::fs::File;
use std::io::{self, Read};
mod divide;
mod guess;
fn main() {
    println!("Start!");
    // let greeting_file = File::open("hello.txt");
    // match greeting_file {
    //     Ok(file) => println!("file : {:?}", file),
    //     // Err(error) => panic!("Problem with opening the file {error:?}"), //a basic case to panic the error.
    //     Err(e) => match e.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(file_created) => println!("file_created : {:?}", file_created),
    //             Err(error) => panic!("problem creating the file : {error:?}"),
    //         },
    //         _ => panic!("problem opening the file {:?}", e),
    //     },
    // };
    // println!("file; {:?}",greeting_file);

    let result = read_username_from_file();
    println!("result : {:?}", result);

    let value = guess::Guess::new(4);
    // println!("guess::new : {:?}", guess::Guess::value(&value));
    println!("value : {:?}", value);
    let quotient = divide::divide(8, 0);
    println!("Quotient : {quotient:?}");

    println!("End!");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
