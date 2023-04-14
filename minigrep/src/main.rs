use minigrep::Config;
use std::env;
use std::process;
fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing argument : {err}");
        process::exit(1);
    });

    if let Err(e) = Config::run(config) {
        eprintln!("Application error {e}");
        process::exit(1);
    }
}
