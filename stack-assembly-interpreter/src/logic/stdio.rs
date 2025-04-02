use console::Term;
use anyhow::{anyhow, Result};

use crate::models::command::{Input, Output};

pub struct Stdio;

impl Input for Stdio {
    fn get_char(&self) -> Result<i64> {
        Ok(Term::stdout().read_char().map(|c| c as i64)?)
    }
}

impl Output for Stdio {
    fn print_char(&self, c: i64) -> Result<()> {
        let c = char::from_u32(u32::try_from(c)?).ok_or_else(|| anyhow!("invalid character code {c}"))?;
        print!("{c}");
        Ok(())
    }
}
