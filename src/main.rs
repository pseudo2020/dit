use std::env;
use config::Config;

mod config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::new(&args).unwrap();

    println!("Original File: {}", config.org_file); 
    println!("New File: {}", config.new_file); 
    println!("Flag: {}", config.flag); 
    
}