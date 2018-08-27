use std::env;
use std::fs;
use std::io::prelude::*;

fn main() {
    println!("Hello, world!");

    // Collect the command line arguments from the  user input
    let args: Vec<String> = env::args().collect();

    // User specifies the query type
    let query = &args[1];
    let file = &args[2];

    println!("Searching for {:?}", query);
    println!("In file: {:?}", file);

    let contents = fs::read_to_string(file)
                        .expect("Error reading this file.");

    println!("With text:\n {:?}", &contents);
}
