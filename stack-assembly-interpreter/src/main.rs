use anyhow::Result;

use std::{env, process::exit};

use stack_assembly_interpreter::run;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_paths = &args[1..];

    let rc = run(&file_paths)?;
    exit(rc.try_into()?)
}

