mod runtime;
use std::env;
use std::fs;
use std::path::{Path};

fn main() {
    let args : Vec<String> = env::args().collect();
    let filepath = Path::new(
        env::current_dir().unwrap().to_str().unwrap()
    ).join(&args[1]);

    println!("Running: {}", filepath.to_str().unwrap());
    let fileContents = fs::read_to_string(filepath).unwrap();

    let result = runtime::run_shallow_scope(&fileContents);

    println!("{}", result.unwrap());
}
