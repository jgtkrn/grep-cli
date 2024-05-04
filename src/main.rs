use std::env;
use std::process;
use grep_cli::Config;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("too few arguments: {}", err);
        process::exit(1);
    });
    println!("searching for {}", config.query);
    println!("in file{}", config.path);

    if let Err(e) = grep_cli::run(config) {
        println!("application error: {e}");
        process::exit(1);
    }
}
