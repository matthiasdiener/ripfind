extern crate walkdir;
extern crate regex;

use walkdir::WalkDir;
use regex::Regex;

use std::env;


fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: ripfind <regex> [dir]");
        std::process::exit(1);
    }

    let regexstr =  &args[1];
    let mut dirstr = ".";

    if args.len() > 2 {
        println!("{:?}", args[2]);
        dirstr =  &args[2];
    }

    let re = Regex::new(regexstr).unwrap();

    for entry in WalkDir::new(dirstr).into_iter().filter_map(|e| e.ok()) {
        if re.is_match(&entry.path().display().to_string()) {
            println!("{}", entry.path().display());
        }
    }
}

