mod logic;
mod models;

use std::fs;

use anyhow::{Context, Result};

use logic::{assembly::{self, TextFile}, stdio::Stdio, vm::Executor};
use models::{command::ReturnCode, vm::VM};

pub fn run(file_paths: &[String]) -> Result<ReturnCode> {
    let files: Result<Vec<TextFile>> = file_paths.into_iter()
                             .map(|path| -> Result<_> {
                                 Ok(TextFile{
                                     name: path.to_string(),
                                     text: fs::read_to_string(path)
                                         .context(format!("failed to read file: {path}"))?
                                 })
                             })
                             .collect();

    let instructions = assembly::assembly(&files?)?;

    let vm = VM::new(instructions);
    let executor = Executor{ io: &Stdio{} };

    executor.execute(vm)
}
