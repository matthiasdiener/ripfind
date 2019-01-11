extern crate walkdir;
extern crate regex;
extern crate getopts;
extern crate colored;
extern crate atty;


use colored::*;
use getopts::Options;
use walkdir::WalkDir;
use regex::Regex;
use regex::Captures;
use std::env;


fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [regex] [dir] [options]", program);
    print!("{}", opts.usage(&brief));
}

fn print_version() {
    const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
    print!("Ripfind {}\n", VERSION.unwrap_or("unknown"));
}


fn add_options(opts: &mut Options) {
    opts.optflag("i", "ignore-case", "Search case insensitively.");
    opts.optflag("s", "sensitive-case", "Search case sensitively.");
    opts.optflag("h", "help", "Print this help menu.");
    opts.optflag("v", "version", "Print version.");
    opts.optopt("", "color", "Color output.", "WHEN");
}


fn parse_options() -> (String, String, bool) {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    add_options(&mut opts);

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(_f) => { print_usage(&program, opts); std::process::exit(1) }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        std::process::exit(0);
    }

    if matches.opt_present("v") {
        print_version();
        std::process::exit(0);
    }

    let color_output = if matches.opt_present("color") {
            match matches.opt_str("color").unwrap().as_ref() {
                "never" => false,
                "always" => true,
                "auto" => atty::is(atty::Stream::Stdout),
                _ => { println!("Error: unknown color mode '{}' specified.\n", matches.opt_str("color").unwrap()); print_usage(&program, opts); std::process::exit(1) }
            }
        } else { atty::is(atty::Stream::Stdout) };

    let ignore_case = if matches.opt_present("i") { true } else { false };

    let re = format!("{}({})", if ignore_case {"(?i)"} else {""}, if matches.free.len() > 0 {matches.free[0].clone()} else {String::from(".")});

    let dir = if matches.free.len() > 1 {matches.free[1].clone()} else {String::from(".")};

    return(re, dir, color_output);
}


fn main() {
    let (re, dir, color_output) = parse_options();

    let re = Regex::new(&re).unwrap();

    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        let string = entry.path().display().to_string();
        if re.is_match(&string) {
            if color_output {
                let output = re.replace_all(&string, |caps: &Captures| { format!("{}", &caps[1].red().bold()) });
                println!("{}", output);
            } else {
                println!("{}", string);
            }
        }
    }
}
