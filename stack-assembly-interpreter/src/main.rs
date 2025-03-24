use std::{env, fs};
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_paths = &args[1..];

    run(&file_paths).unwrap();
}

fn run(file_paths: &[String]) -> Result<(), Box<dyn Error>> {
    let contents: Result<Vec<_>, _> = file_paths.iter().map(fs::read_to_string).collect();

    println!("Contents: |{}|", contents?.concat());

    Ok(())
}
