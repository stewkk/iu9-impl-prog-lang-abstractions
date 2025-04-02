use anyhow::Result;

use std::{env, process::ExitCode};

use stack_assembly_interpreter::run;

fn main() -> Result<ExitCode> {
    let args: Vec<String> = env::args().collect();
    let file_paths = &args[1..];

    let rc = run(&file_paths)?;
    Ok(ExitCode::from(u8::try_from(rc)?))
}
