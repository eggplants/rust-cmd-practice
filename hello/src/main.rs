use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Hello, world!");
    } else {
        println!("Hello, world, [{}]!", &args[1..].join(","));
    }
    exit(0);
}
