extern crate walkdir;
extern crate regex;
extern crate getopts;

use getopts::Options;
use walkdir::WalkDir;
use regex::Regex;
use std::env;


fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} <regex> [dir] [options]", program);
    print!("{}", opts.usage(&brief));
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("i", "ignore-case", "When this flag is provided, the given patterns will be searched case insensitively.");
    opts.optflag("h", "help", "Print this help menu.");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(_f) => { print_usage(&program, opts); std::process::exit(1)}
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let re = if !matches.free.is_empty() {
        if matches.opt_present("i") {
            format!("{}{}", "(?i)", matches.free[0].clone())
        } else {
           matches.free[0].clone()
        }
    } else {
        print_usage(&program, opts);
        std::process::exit(1);
    };

    let dirstr = if matches.free.len() > 1 {
        matches.free[1].clone()
    } else {
        // Search in current directory by default
        String::from(".")
    };

    let re = Regex::new(&re).unwrap();

    for entry in WalkDir::new(dirstr).into_iter().filter_map(|e| e.ok()) {
        if re.is_match(&entry.path().display().to_string()) {
            println!("{}", entry.path().display());
        }
    }
}

