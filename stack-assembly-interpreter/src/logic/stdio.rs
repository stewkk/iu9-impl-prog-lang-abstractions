use anyhow::{anyhow, Result};

use std::io::{self, BufRead};

use crate::models::command::{Input, Output};

pub struct Stdio {
    buffer: String,
}

impl Stdio {
    pub fn new() -> Self {
        Self { buffer: "".to_string() }
    }
}

impl Input for Stdio {
    fn get_char(&mut self) -> Result<i64> {
        while self.buffer.is_empty() {
            let mut stdin = io::stdin().lock();
            stdin.read_line(&mut self.buffer)?;
        }
        let c = self.buffer.chars().next().ok_or_else(|| anyhow!("empty buffer"))?;
        self.buffer = self.buffer.get(1..).unwrap_or_default().to_string();

        Ok(c as i64)
    }
}

impl Output for Stdio {
    fn print_char(&self, c: i64) -> Result<()> {
        let c = char::from_u32(u32::try_from(c)?).ok_or_else(|| anyhow!("invalid character code {c}"))?;
        print!("{c}");
        Ok(())
    }
}
