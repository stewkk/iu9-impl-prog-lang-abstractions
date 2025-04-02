mod logic;
mod models;

use std::{error, fs};

use anyhow::Result;

use logic::assembly::{self, TextFile};
use models::vm::VM;

pub fn run(file_paths: &[String]) -> Result<(), Box<dyn error::Error>> {
    let files: Result<Vec<TextFile>> = file_paths.into_iter()
                             .map(|path| -> Result<_> {
                                 Ok(TextFile{name: path.to_string(), text: fs::read_to_string(path)?})
                             })
                             .collect();

    let instructions = assembly::assembly(&files?)?;

    let vm = VM::new(instructions);

    // TODO: execute instructions

    Ok(())
}

