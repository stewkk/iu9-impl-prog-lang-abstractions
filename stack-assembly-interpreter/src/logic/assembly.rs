use std::collections::HashMap;

use anyhow::{anyhow, Result};

use crate::models::token::Token;
use crate::models::command::Opcode;
use super::tokenize;
use super::labels;

fn generate_opcodes(tokens: &[Token], labels: HashMap<&str, Opcode>) -> Result<Vec<Opcode>> {
    tokens.iter()
          .filter_map(|x| match x {
              Token::Declaration(_, _) => None,
              _ => Some(x),
          })
          .map(|x| match x {
              Token::Ident(i, _) => labels.get(i.as_str()).copied().ok_or_else(|| anyhow!("undefined ident: {i}")),
              Token::Integer(i, _) => Ok(*i),
              Token::Declaration(_, _) => Err(anyhow!("didn't expect declaration here")),
          })
          .collect()
}

pub struct TextFile<'a> {
    name: &'a str,
    text: &'a str,
}

pub fn assembly(files: &[TextFile]) -> Result<Vec<Opcode>> {
    let tokens_by_file: Result<Vec<Vec<Token>>> = files.iter().map(|file| tokenize::tokenize(file.text, file.name)).collect();
    let tokens: Vec<Token> = tokens_by_file?.into_iter().flatten().collect();
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

        let got = assembly(&[TextFile{name: "test", text}]);

        assert_eq!(got.unwrap(), vec![10, 65, -40, 268, -37, 268, 268, 259, 260, 123, 1234, 268]);
    }

    #[ignore="not implemented"]
    #[test]
    fn translates_hello_world() {
        let text = "72 OUT 101 OUT 108 OUT 108 OUT 111 OUT 33 OUT 0 HALT";

        let got = assembly(&[TextFile{name: "test", text}]);

        assert_eq!(got.unwrap(), vec![72, -44, 101, -44, 108, -44, 108, -44, 111, -44, 33, -44, 0, -37]);
    }

    #[test]
    fn translates_commands() {
        let text = "72 0 ADD";

        let got = assembly(&[TextFile{name: "test", text}]);

        assert_eq!(got.unwrap(), vec![72, 0, -1]);
    }

    #[test]
    fn translates_labels() {
        let text = "72 :a a 123 a";

        let got = assembly(&[TextFile{name: "test", text}]);

        assert_eq!(got.unwrap(), vec![72, 257, 123, 257]);
    }

    #[test]
    fn error_on_undefined_ident() {
        let text = "72 a 123 a";

        let got = assembly(&[TextFile{name: "test", text}]);

        assert_eq!(got.unwrap_err().to_string(), "undefined ident: a");
    }
}
