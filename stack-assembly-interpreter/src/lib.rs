mod logic;
mod models;

use std::{collections::HashMap, error, fs};

use anyhow::Result;

use logic::{labels, tokenize};
use models::token::Token;

pub fn run(file_paths: &[String]) -> Result<(), Box<dyn error::Error>> {
    let contents: Result<Vec<_>, _> = file_paths.iter().map(fs::read_to_string).collect();

    println!("Tokenized: {:?}", tokenize::tokenize(contents?.concat().as_str()));

    Ok(())
}

type Opcode = i32;

fn generate_opcodes(tokens: &[Token], labels: HashMap<&str, i32>) -> Vec<Opcode> {
    tokens.iter().filter_map(|x| match x {
        Token::Integer(i) => Some(*i),
        Token::Declaration(_) => None,
        Token::Ident(i) => labels.get(i.as_str()).copied(),
    }).collect()
}

fn assembly(text: &str) -> Result<Vec<Opcode>> {
    let tokens = tokenize::tokenize(text)?;
    let labels = labels::get_labels(&tokens)?;
    Ok(generate_opcodes(&tokens, labels))
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn translates_hello_world() {
        let text = "72 OUT 101 OUT 108 OUT 108 OUT 111 OUT 33 OUT 0 HALT";

        let got = assembly(text);

        assert_eq!(got.unwrap(), vec![72, -44, 101, -44, 108, -44, 108, -44, 111, -44, 33, -44, 0, -37]);
    }

    #[test]
    fn translates_add() {
        let text = "72 0 ADD";

        let got = assembly(text);

        assert_eq!(got.unwrap(), vec![72, 0, -1]);
    }

    // TODO: error cases
}
