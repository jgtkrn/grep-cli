use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect(); 
    let query: &str = &args[1];
    let path: &str = &args[2];

    println!("searching for {}", query);
    println!("in file{}", path);
    let contents = fs::read_to_string(path).expect("should have to read now.");
    println!("with contents: \r\n {}", contents);

}
