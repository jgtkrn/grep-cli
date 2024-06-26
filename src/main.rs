use std::env;
use std::process;
use grep_cli_lin::Config;

#[allow(unused_variables)]
fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing argument: {err}");
        process::exit(1);
    });

    if let Err(e) = grep_cli_lin::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
