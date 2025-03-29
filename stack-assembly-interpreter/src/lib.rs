mod logic;
mod models;

use std::{error, fs};

use anyhow::Result;

use logic::tokenize;

pub fn run(file_paths: &[String]) -> Result<(), Box<dyn error::Error>> {
    let contents: Result<Vec<_>, _> = file_paths.iter().map(fs::read_to_string).collect();

    println!("Tokenized: {:?}", tokenize::tokenize(contents?.concat().as_str(), "stdin")?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
}
