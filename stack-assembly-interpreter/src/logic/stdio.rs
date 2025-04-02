use super::vm::{Input, Output};

pub struct Stdio;

impl Input for Stdio {
    fn get_char(&self) -> i64 {
        todo!()
    }
}

impl Output for Stdio {
    fn print_char(c: i64) {
        todo!()
    }
}
