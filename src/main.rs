use crate::options::parse_options;
use colored::*;
use regex::Captures;
use regex::Regex;
use walkdir::DirEntry;
use walkdir::WalkDir;

mod options;

fn filter_filetype(entry: &DirEntry, filetype: &str) -> bool {
    match filetype {
        "" => true,
        "d" => entry.file_type().is_dir(),
        "f" => entry.file_type().is_file(),
        "l" => entry.file_type().is_symlink(),
        _ => {
            println!("Error: unknown file type '{}' specified.\n", filetype);
            std::process::exit(1)
        }
    }
}

fn main() {
    let (re, dir, color_output, filetype) = parse_options();

    let re = Regex::new(&re).unwrap();

    let walker = WalkDir::new(dir).into_iter();

    for entry in walker.filter_map(|e| e.ok()) {
        if !filter_filetype(&entry, &filetype) {
            continue;
        }
        let string = &entry.path().display().to_string();
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
