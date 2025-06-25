use std::env;
use std::fs;

fn main() {
    println!("Start!");
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let args = Args::new(&args);

    println!("searching for : {}", args.query);
    println!("In file {}", args.file_path);

    let content = fs::read_to_string(args.file_path).expect("something went wrong");

    println!("Content : {}", content);

    println!("End!");
}

struct Args {
    query: String,
    file_path: String,
}

impl Args {
    fn new(args: &[String]) -> Args {
        let query = args[1].clone();
        let file_path = args[2].clone();

        Args { query, file_path }
    }
}


