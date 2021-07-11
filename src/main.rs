use std::env;
//use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::new(&args).unwrap();

    println!("Original File: {}", config.org_file); 
    println!("New File: {}", config.new_file); 
    println!("Flag: {}", config.flag); 
    
}

struct Config {
    flag: bool,         // true -> generate diff, false -> patch
    org_file: String,
    new_file: String,
} impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        let flag: bool;
        let new_file: String;

        let args_len = args.len();
        
        if args_len < 2 {
            return Err("Not enough arguments!");
        }

        let org_file: String = args[2].clone();

        match args[1].chars().nth(0).unwrap() {
            'd' => flag = true,
            'p' => flag = false,
            _ => return Err("Invalid argument"),
        }

        if flag && args_len == 4 {    // if generate diff
            new_file = args[3].clone();
        } else if !flag {               // if patch
            new_file = String::new();
        } else {
            return Err("File path arguments error");
        }

        Ok(Config { flag, org_file, new_file })
    }
}
