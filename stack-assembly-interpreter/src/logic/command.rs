use crate::models::command::Command;

pub static COMMANDS: &[Option<Command>] = &[
    None,
    Some(Command{mnemonics: &["ADD"], handler: add_handler}),
    Some(Command{mnemonics: &["SUB"], handler: sub_handler}),
];

fn add_handler() -> () {}

fn sub_handler() -> () {}

#[cfg(test)]
mod tests {
    use super::*;


}
