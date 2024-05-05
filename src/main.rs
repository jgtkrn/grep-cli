use std::env;
use std::process;
use grep_cli::Config;

#[allow(unused_variables)]
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing argument: {err}");
        process::exit(1);
    });

    if let Err(e) = grep_cli::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
