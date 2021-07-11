use std::env;
//use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::new(&args).unwrap();

    if config.flag { println!("{}", config.filename); }
}

struct Config {
    flag: bool,         // true -> generate diff, false -> patch
    orgFile: String,
    newFile: String,
} impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
	let flag: bool;

	if args.len() > 3 {
	    return Err("Too many arguments!");
	} else if args.len() <  {
	     return Err("Not enough arguments!");
	}

	match args[2].chars().nth(0).unwrap() {
	    d => flag = true,
	    p => flag = false,
	    _ => return Err("Invalid argument"),
	}

	let filename: String = args[2].clone();
	Ok(Config { flag, orgFile, newFile })
    }
}
