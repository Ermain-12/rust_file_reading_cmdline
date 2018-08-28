use std::env;
use std::fs;
use std::process;


fn main() {
    println!("Hello, world!");

    // Collect the command line arguments from the  user input
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error parsing arugments: {:?}", err);
        process::exit(1);
    });

    run(config);
}


fn run(config: Config) {
    let contents = fs::read_to_string(config.file_name)
                        .expect("Error reading this file");

    println!("With text: \n{:?}", contents);
}


#[derive(Debug)]
struct Config {
    query: String,
    file_name: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 3 {
            return Err("Not enough arguments!");
        }

        let query = args[1].clone();
        let file_name = args[2].clone();

        Ok(Config { query, file_name })
    }
}
