use std::env;
use std::fs;
use grep_cli::Config;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    println!("searching for {}", config.query);
    println!("in file{}", config.path);
    let contents = fs::read_to_string(config.path).expect("should have to read now.");
    println!("with contents: \r\n {}", contents);

}
