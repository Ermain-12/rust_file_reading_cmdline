use std::env;
use std::fs;
use std::io::prelude::*;

fn main() {
    println!("Hello, world!");

    // Collect the command line arguments from the  user input
    let args: Vec<String> = env::args().collect();

    // User specifies the query type
    let query = &args[1];
    let file_name = &args[2];

    println!("Searching for {:?}", query);
    println!("In file: {:?}", file_name);

    let contents = fs::read_to_string(file_name)
                        .expect("Error reading this file.");

    println!("With text:\n {:?}", contents);
}


fn parse(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}

#[derive(Debug)]
struct Config {
    query: String,
    filename: String,
}
