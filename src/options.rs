use getopts::Options;
use std::env;
use std::io::IsTerminal;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [regex] [dir] [options]", program);
    print!("{}", opts.usage(&brief));
}

fn print_version() {
    const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
    println!("Ripfind v{}", VERSION.unwrap_or("unknown"));
}

fn add_options(opts: &mut Options) {
    opts.long_only(true);
    opts.optflag("i", "ignore-case", "Search case insensitively.");
    opts.optflag("s", "sensitive-case", "Search case sensitively.");
    opts.optflag("h", "help", "Print this help menu.");
    opts.optflag("v", "version", "Print version.");
    opts.optopt(
        "",
        "color",
        "Color output.\nWHEN can be never, always, or auto.",
        "<WHEN>",
    );
    opts.optopt(
        "",
        "type",
        "Filter file type.\n<TYPE> can be:\nd - directory\nf - regular file\nl - symbolic link.",
        "<TYPE>",
    );
}

pub fn parse_options() -> (String, String, bool, String) {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    add_options(&mut opts);

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(_f) => {
            print_usage(&program, opts);
            std::process::exit(1)
        }
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
            "auto" => std::io::stdout().is_terminal(),
            _ => {
                println!(
                    "Error: unknown color mode '{}' specified.\n",
                    matches.opt_str("color").unwrap()
                );
                print_usage(&program, opts);
                std::process::exit(1)
            }
        }
    } else {
        std::io::stdout().is_terminal()
    };

    let ignore_case = matches.opt_present("i");

    let re = format!(
        "{}({})",
        if ignore_case { "(?i)" } else { "" },
        if !matches.free.is_empty() {
            matches.free[0].clone()
        } else {
            String::from(".")
        }
    );

    let dir = if matches.free.len() > 1 {
        matches.free[1].clone()
    } else {
        String::from(".")
    };

    let filetype = if matches.opt_present("type") {
        match matches.opt_str("type").unwrap().as_ref() {
            "f" => String::from("f"),
            "d" => String::from("d"),
            "l" => String::from("l"),
            _ => {
                println!(
                    "Error: unknown file type '{}' specified.\n",
                    matches.opt_str("type").unwrap()
                );
                print_usage(&program, opts);
                std::process::exit(1)
            }
        }
    } else {
        String::from("")
    };

    (re, dir, color_output, filetype)
}
