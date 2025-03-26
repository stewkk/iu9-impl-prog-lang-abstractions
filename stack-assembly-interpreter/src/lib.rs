mod logic;
mod models;

use std::{collections::HashMap, error, fs};

use anyhow::{anyhow, Result, Error};

use logic::{labels, tokenize};
use models::token::Token;

pub fn run(file_paths: &[String]) -> Result<(), Box<dyn error::Error>> {
    let contents: Result<Vec<_>, _> = file_paths.iter().map(fs::read_to_string).collect();

    println!("Tokenized: {:?}", tokenize::tokenize(contents?.concat().as_str()));

    Ok(())
}

type Opcode = i32;

fn generate_opcodes(tokens: &[Token], labels: HashMap<&str, i32>) -> Result<Vec<Opcode>> {
    tokens.iter()
          .filter_map(|x| match x {
              Token::Declaration(_) => None,
              _ => Some(x),
          })
          .map(|x| match x {
              Token::Ident(i) => labels.get(i.as_str()).copied().ok_or_else(|| anyhow!("undefined ident: {i}")),
              Token::Integer(i) => Ok(*i),
              Token::Declaration(_) => Err(anyhow!("didn't expect declaration here")),
          })
          .collect()
}

fn assembly(text: &str) -> Result<Vec<Opcode>> {
    let tokens = tokenize::tokenize(text)?;
    let labels = labels::get_labels(&tokens)?;
    generate_opcodes(&tokens, labels)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore="not implemented"]
    #[test]
    fn translates_assembly_into_opcodes() {
        let text = "10 +65 -40 :_  ; _ == 259
_Loop :a1 HALT _Read_number_ _- _ a1 ; a1 == 260
123 ; ;; i'm comment
1234 PROGRAM_SIZE
:_Loop :_Read_number_ :_- ; _Loop == _Read_number_ == _- == 268";

        let got = assembly(text);

        assert_eq!(got.unwrap(), vec![10, 65, -40, 268, -37, 268, 268, 259, 260, 123, 1234, 268]);
    }

    #[ignore="not implemented"]
    #[test]
    fn translates_hello_world() {
        let text = "72 OUT 101 OUT 108 OUT 108 OUT 111 OUT 33 OUT 0 HALT";

        let got = assembly(text);

        assert_eq!(got.unwrap(), vec![72, -44, 101, -44, 108, -44, 108, -44, 111, -44, 33, -44, 0, -37]);
    }

    #[test]
    fn translates_commands() {
        let text = "72 0 ADD";

        let got = assembly(text);

        assert_eq!(got.unwrap(), vec![72, 0, -1]);
    }

    #[test]
    fn translates_labels() {
        let text = "72 :a a 123 a";

        let got = assembly(text);

        assert_eq!(got.unwrap(), vec![72, 257, 123, 257]);
    }

    #[test]
    fn error_on_undefined_ident() {
        let text = "72 a 123 a";

        let got = assembly(text);

        assert_eq!(got.unwrap_err().to_string(), "undefined ident: a");
    }
}
