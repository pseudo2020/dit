use std::env;
use config::Config;
use std::fs::File;
use std::io::{prelude::*, BufReader};

mod config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::new(&args).unwrap();
    let mut org_file = File::open(config.org_path).expect("Cannot open file!");

    // Used only in creating diffs
    if config.flag {
	let mut new_file = File::open(config.new_path).expect("Cannot open file!");
    }
    
    parse_diff(org_file);
}

/*
fn generate_diff(org_file: File, new_file: File) -> Result<File, &'static str> {
    let mut insertions: i32;
    let mut deletions: i32;
    
}
*/

fn patch_file(org_file: File) -> () {
    ()
}

fn parse_diff(diff_file: File) {
    let modified_file: File;
    let total_modification: i32;
    let insertions: i32;
    let deletions: i32;
    let reader = BufReader::new(diff_file);
    for line in reader.lines() {
	println!("{}", line.unwrap());
    }
}
