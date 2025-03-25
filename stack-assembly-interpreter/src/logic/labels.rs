use std::collections::HashMap;

use anyhow::{bail, Result};

use super::command::COMMANDS;
use crate::models::token::Token;

fn get_default_labels() -> HashMap<&'static str, i32> {
    COMMANDS.iter().map(|x| (x.mnemonic, x.code)).collect()
}

pub fn get_labels(tokens: &[Token]) -> Result<HashMap<&str, i32>> {
    let mut current = 256;
    let mut labels = get_default_labels();
    for token in tokens {
        if let Token::Declaration(decl) = token {
            if let Some(_) = labels.insert(decl, current) {
                bail!("label declared twice: {decl}");
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

    #[test]
    fn calculates_labels_map() {
        let tokens = vec![
            Token::Integer(-40),
            Token::Ident("_Loop".to_string()),
            Token::Declaration("a1".to_string()),
            Token::Ident("HALT".to_string()),
            Token::Ident("_Read_number_".to_string()),
            Token::Ident("a1".to_string()),
            Token::Integer(123),
            Token::Integer(1234),
            Token::Ident("PROGRAM_SIZE".to_string()),
            Token::Declaration("_Loop".to_string()),
            Token::Declaration("_Read_number_".to_string()),
        ];
        let mut expected = get_default_labels();
        expected.insert("_Loop", 264);
        expected.insert("a1", 258);
        expected.insert("_Read_number_", 264);
        expected.insert("PROGRAM_SIZE", 264);

        let got = get_labels(&tokens);

        assert_eq!(got.unwrap(), expected)
    }

    // TODO: label defined twice test
    #[test]
}
