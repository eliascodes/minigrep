extern crate minigrep;

use minigrep::*;
use std::env;
use std::process;

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = main::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
