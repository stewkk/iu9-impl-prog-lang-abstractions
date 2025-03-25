mod logic;
mod models;

use std::{collections::HashMap, error, fs};

use anyhow::{bail, Result};

use logic::tokenize;
use models::token::Token;

pub fn run(file_paths: &[String]) -> Result<(), Box<dyn error::Error>> {
    let contents: Result<Vec<_>, _> = file_paths.iter().map(fs::read_to_string).collect();

    println!("Tokenized: {:?}", tokenize::tokenize(contents?.concat().as_str()));

    Ok(())
}

type CommandHandler = fn() -> ();

struct Command<'a> {
    code: i32,
    mnemonic: &'a str,
    handler: CommandHandler,
}

fn AddHandler() -> () {}

fn SubHandler() -> () {}

static COMMANDS: &[Command] = &[
    Command{code: -1, mnemonic: "ADD", handler: AddHandler},
    Command{code: -2, mnemonic: "SUB", handler: SubHandler},
];

fn get_default_labels() -> HashMap<&'static str, i32> {
    COMMANDS.iter().map(|x| (x.mnemonic, x.code)).collect()
}

fn get_labels(tokens: &[Token]) -> Result<HashMap<&str, i32>> {
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

fn _assembly(_text: &str) -> Vec<i32> {
    vec![]
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

    #[ignore = "not implemented"]
    #[test]
    fn translates_assembly_into_opcodes() {
        let text = "10 +65 -40 :_  ; _ == 259
_Loop :a1 HALT _Read_number_ _- _ a1 ; a1 == 260
123 ; ;; i'm comment
1234 PROGRAM_SIZE
:_Loop :_Read_number_ :_- ; _Loop == _Read_number_ == _- == 268";

        let got = _assembly(text);

        assert_eq!(got, vec![10, 65, -40, 268, -37, 268, 268, 259, 260, 123, 1234, 268]);
    }

    #[ignore = "not implemented"]
    #[test]
    fn translates_hello_world() {
        let text = "72 OUT 101 OUT 108 OUT 108 OUT 111 OUT 33 OUT 0 HALT";

        let got = _assembly(text);

        assert_eq!(got, vec![72, -44, 101, -44, 108, -44, 108, -44, 111, -44, 33, -44, 0, -37]);
    }
}
