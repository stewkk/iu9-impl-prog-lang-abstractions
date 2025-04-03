use std::collections::HashMap;

use anyhow::{anyhow, Result};
use beau_collector::BeauCollector as _;

use crate::models::token::Token;
use crate::models::command::{Opcode, Instruction};
use super::tokenize;
use super::labels;

fn generate_instructions(tokens: &[Token], labels: HashMap<&str, Opcode>) -> Result<Vec<Instruction>> {
    tokens.iter()
          .filter_map(|x| match x {
            Token::Declaration(_, _) => None,
            _ => Some(x),
          })
          .map(|x| match x {
            Token::Ident(i, pos) => labels.get(i.as_str())
                                          .copied()
                                          .map(|opcode| Instruction{opcode, token: x.clone()})
                                          .ok_or_else(|| anyhow!("{pos}: undefined ident: \"{i}\"")),
            Token::Integer(i, _) => Ok(Instruction{opcode: *i, token: x.clone()}),
            Token::Declaration(_, pos) => Err(anyhow!("{pos}: didn't expect declaration here")),
          })
          .bcollect()
}

pub struct TextFile {
    pub name: String,
    pub text: String,
}

pub fn assembly(files: &[TextFile]) -> Result<Vec<Instruction>> {
    let tokens_by_file: Result<Vec<Vec<Token>>> = files.iter()
                                                       .map(|file| tokenize::tokenize(&file.text, &file.name))
                                                       .bcollect::<Vec<_>>();
    let tokens: Vec<Token> = tokens_by_file?.into_iter()
                                            .flatten()
                                            .collect();
    let labels = labels::get_labels(&tokens)?;
    generate_instructions(&tokens, labels)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translates_assembly_into_opcodes() {
        let text = String::from("10 +65 -40 :_  ; _ == 259
_Loop :a1 HALT _Read_number_ _- _ a1 ; a1 == 260
123 ; ;; i'm comment
1234 PROGRAM_SIZE
:_Loop :_Read_number_ :_- ; _Loop == _Read_number_ == _- == 268");

      let got = assembly(&[TextFile{name: "test".to_owned(), text}]).unwrap()
                                                         .iter()
                                                         .map(|x| x.opcode)
                                                         .collect::<Vec<_>>();

        assert_eq!(got, vec![10, 65, -40, 268, -37, 268, 268, 259, 260, 123, 1234, 268]);
    }

    #[test]
    fn translates_hello_world() {
        let text = String::from("72 OUT 101 OUT 108 OUT 108 OUT 111 OUT 33 OUT 0 HALT");

      let got = assembly(&[TextFile{name: "test".to_owned(), text}]).unwrap()
                                                         .iter()
                                                         .map(|x| x.opcode)
                                                         .collect::<Vec<_>>();

        assert_eq!(got, vec![72, -44, 101, -44, 108, -44, 108, -44, 111, -44, 33, -44, 0, -37]);
    }

    #[test]
    fn translates_commands() {
        let text = String::from("72 0 ADD");

      let got = assembly(&[TextFile{name: "test".to_owned(), text}]).unwrap()
                                                         .iter()
                                                         .map(|x| x.opcode)
                                                         .collect::<Vec<_>>();

        assert_eq!(got, vec![72, 0, -1]);
    }

    #[test]
    fn translates_labels() {
        let text = String::from("72 :a a 123 a");

      let got = assembly(&[TextFile{name: "test".to_owned(), text}]).unwrap()
                                                         .iter()
                                                         .map(|x| x.opcode)
                                                         .collect::<Vec<_>>();

        assert_eq!(got, vec![72, 257, 123, 257]);
    }

    #[test]
    fn error_on_undefined_ident() {
        let text = String::from("72 a 123 a");

        let got = assembly(&[TextFile{name: "test".to_owned(), text}]);

        assert_eq!(got.unwrap_err().to_string(), "test:1:4: undefined ident: \"a\"");
    }
}
