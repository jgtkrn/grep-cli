use std::fs;
use std::error::Error;

#[allow(dead_code)]
pub struct Config {
    pub query: String,
    pub path: String
}

#[allow(dead_code)]
impl Config {
    pub fn build (args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {return Err("please use proper arguments.")};
        let query: String = args[1].clone();
        let path: String = args[2].clone();
        Ok(Config{query,path})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.path)?;
    println!("with contents: \r\n {}", contents);
    Ok(())
}