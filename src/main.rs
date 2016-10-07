use std::env;

fn main() {
    let mut args = env::args();

    match args.nth(1) {
        None => {
            println!("Usage: fuzzy PATTERN [FILE...]");
            std::process::exit(1);
        },
        Some(pattern) => {
            println!("pattern = {}", pattern);
        }
    }
    // TODO: everything
}
