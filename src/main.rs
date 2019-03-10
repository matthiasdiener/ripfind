use crate::options::parse_options;
use colored::*;
use regex::Captures;
use regex::Regex;
use walkdir::WalkDir;

mod options;

fn main() {
    let (re, dir, color_output) = parse_options();

    let re = Regex::new(&re).unwrap();

    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        let string = entry.path().display().to_string();
        if re.is_match(&string) {
            if color_output {
                let output = re.replace_all(&string, |caps: &Captures| {
                    format!("{}", &caps[1].red().bold())
                });
                println!("{}", output);
            } else {
                println!("{}", string);
            }
        }
    }
}
