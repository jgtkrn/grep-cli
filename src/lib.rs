#[allow(dead_code)]
pub struct Config {
    pub query: String,
    pub path: String
}

#[allow(dead_code)]
impl Config {
    pub fn new (args: &[String]) -> Config {
        let query: String = args[1].clone();
        let path: String = args[2].clone();
        Config{query,path}
    }
} 