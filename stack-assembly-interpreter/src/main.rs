use std::env;

use stack_assembly_interpreter::run;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_paths = &args[1..];

    run(&file_paths).unwrap();
}

