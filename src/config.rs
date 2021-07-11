pub struct Config {
    pub flag: bool,         // true -> generate diff, false -> patch
    pub org_path: String,
    pub new_path: String,
} impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        let flag: bool;
        let new_path: String;

        let args_len = args.len();
        
        if args_len < 2 {
            return Err("Not enough arguments!");
        }

        let org_path: String = args[2].clone();

        match args[1].chars().nth(0).unwrap() {
            'd' => flag = true,
            'p' => flag = false,
            _ => return Err("Invalid argument"),
        }

        if flag && args_len == 4 {    // if generate diff
            new_path = args[3].clone();
        } else if !flag {               // if patch
            new_path = String::new();
        } else {
            return Err("File path arguments error");
        }

        Ok(Config { flag, org_path, new_path })
    }
}
