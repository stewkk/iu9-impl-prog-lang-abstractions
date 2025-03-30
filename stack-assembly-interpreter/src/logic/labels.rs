use std::collections::HashMap;

use anyhow::{bail, Result};

use super::command::COMMANDS;
use crate::models::token::Token;
use crate::models::command::Opcode;

fn get_default_labels() -> HashMap<&'static str, Opcode> {
    COMMANDS.iter()
            .enumerate()
            .filter_map(
                |(i, cmd_opt)| cmd_opt.as_ref()
                                      .map(
                                          |cmd| cmd.mnemonics.iter()
                                                             .map(|mnemonic| (*mnemonic, -(i as Opcode)))
                                                             .collect::<Vec<_>>()
                                      )
            )
            .flatten()
            .collect()
}

pub fn get_labels<'a>(tokens: &'a [Token]) -> Result<HashMap<&'a str, Opcode>> {
    let mut current = 256;
    let mut labels = get_default_labels();
    for token in tokens {
        if let Token::Declaration(decl, pos) = token {
            if let Some(_) = labels.insert(decl, current) {
                bail!("{pos}: label declared twice: {decl}");
            }
        } else {
            current += 1;
        }
    }
    labels.insert("PROGRAM_SIZE", current);
    Ok(labels)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::token::Position;

    #[test]
    fn calculates_labels_map() {
        let tokens = vec![
            Token::Integer(-40, Position{filename: "test".to_string(), line: 1, column: 1}),
            Token::Ident("_Loop".to_string(), Position{filename: "test".to_string(), line: 1, column: 1}),
            Token::Declaration("a1".to_string(), Position{filename: "test".to_string(), line: 1, column: 1}),
            Token::Ident("HALT".to_string(), Position{filename: "test".to_string(), line: 1, column: 1}),
            Token::Ident("_Read_number_".to_string(), Position{filename: "test".to_string(), line: 1, column: 1}),
            Token::Ident("a1".to_string(), Position{filename: "test".to_string(), line: 1, column: 1}),
            Token::Integer(123, Position{filename: "test".to_string(), line: 1, column: 1}),
            Token::Integer(1234, Position{filename: "test".to_string(), line: 1, column: 1}),
            Token::Ident("PROGRAM_SIZE".to_string(), Position{filename: "test".to_string(), line: 1, column: 1}),
            Token::Declaration("_Loop".to_string(), Position{filename: "test".to_string(), line: 1, column: 1}),
            Token::Declaration("_Read_number_".to_string(), Position{filename: "test".to_string(), line: 1, column: 1}),
        ];
        let mut expected = get_default_labels();
        expected.insert("_Loop", 264);
        expected.insert("a1", 258);
        expected.insert("_Read_number_", 264);
        expected.insert("PROGRAM_SIZE", 264);

        let got = get_labels(&tokens);

        assert_eq!(got.unwrap(), expected)
    }

    #[test]
    fn error_on_duplicate_declaration() {
        let tokens = vec![
            Token::Declaration("a1".to_string(), Position{filename: "test".to_string(), line: 1, column: 1}),
            Token::Integer(123, Position{filename: "test".to_string(), line: 1, column: 2}),
            Token::Declaration("a1".to_string(), Position{filename: "test".to_string(), line: 1, column: 3}),
        ];

        let got = get_labels(&tokens);

        assert_eq!(got.unwrap_err().to_string(), "test:1:3: label declared twice: a1");
    }
}
