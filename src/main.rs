use std::env;
use std::process;

use minigrep_yibozhuang::Config;
use minigrep_yibozhuang::run;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Appleication error: {}", e);
        process::exit(1);
    }
}
