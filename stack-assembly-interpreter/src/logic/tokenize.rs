use anyhow::{anyhow, Result, Context, Error};
use once_cell::unsync::Lazy;
use regex::Regex;
use beau_collector::BeauCollector as _;

use crate::models::token::Token;
use crate::models::token::Position;

fn failed_to_tokenize_error(token_type: &str, token_str: &str, pos: &Position) -> Error {
    anyhow!("{pos}: failed to tokenize {token_type}: \"{token_str}\"")
}

fn get_token(token_str: &str, pos: Position) -> Result<Token> {
    let ident_re = Lazy::new(|| Regex::new(r"^[[:alpha:]_][[:word:]-]*$").unwrap());
    let declaration_re = Lazy::new(|| Regex::new(r"^:[[:alpha:]_][[:word:]-]*$").unwrap());

    match token_str.chars().next() {
        Some('a'..='z' | 'A'..='Z' | '_') => ident_re.is_match(token_str)
                                                     .then_some(Token::Ident(token_str.to_string(), pos.clone()) )
                                                     .ok_or_else(|| failed_to_tokenize_error("ident", token_str, &pos)),
        Some('0'..='9' | '+' | '-') => token_str.parse::<i64>().map(|i| Token::Integer(i, pos.clone()))
                                                               .with_context(|| failed_to_tokenize_error("integer", token_str, &pos)),
        Some(':') => declaration_re.is_match(token_str)
                                   .then_some(Token::Declaration(token_str[1..].to_string(), pos.clone()))
                                   .ok_or_else(|| failed_to_tokenize_error("declaration", token_str, &pos)),
        Some(x) => Err(anyhow!("{pos}: token starts with illegal symbol: \"{x}\"")),
        None => Err(anyhow!("{pos}: no valid symbol"))
    }
}

pub fn tokenize(text: &str, filename: &str) -> Result<Vec<Token>> {
    let lines = text.lines().filter_map(|x| x.split(';').next()).enumerate();
    let lines = lines.map(|x| {
        let (i, line) = x;
        (i+1, line.split(&[' ', '\t']))
    });

    let mut res: Vec<Result<Token>> = vec![];
    for line in lines {
        let (i, tokens) = line;
        let mut column: usize = 1;
        for token in tokens {
            if token.is_empty() {
                column += 1;
                continue;
            }
            res.push(get_token(token, Position{filename: filename.to_string(), line: i, column}));
            column += token.len()+1;
        }
    }

    res.into_iter().bcollect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizes_text() {
        let text = "10 +65 -40 :_  ; _ == 259
_Loop :a1 HALT _Read_number_ _- _ a1 ; a1 == 260
123 ; ;; i'm comment
1234 PROGRAM_SIZE
:_Loop :_Read_number_ :_- ; _Loop == _Read_number_ == _- == 268";

        let got = tokenize(text, "test");

        assert_eq!(got.unwrap(), vec![
            Token::Integer(10, Position{filename: "test".to_string(), line: 1, column: 1}),
            Token::Integer(65, Position{filename: "test".to_string(), line: 1, column: 4}),
            Token::Integer(-40, Position{filename: "test".to_string(), line: 1, column: 8}),
            Token::Declaration("_".to_string(), Position{filename: "test".to_string(), line: 1, column: 12}),
            Token::Ident("_Loop".to_string(), Position{filename: "test".to_string(), line: 2, column: 1}),
            Token::Declaration("a1".to_string(), Position{filename: "test".to_string(), line: 2, column: 7}),
            Token::Ident("HALT".to_string(), Position{filename: "test".to_string(), line: 2, column: 11}),
            Token::Ident("_Read_number_".to_string(), Position{filename: "test".to_string(), line: 2, column: 16}),
            Token::Ident("_-".to_string(), Position{filename: "test".to_string(), line: 2, column: 30}),
            Token::Ident("_".to_string(), Position{filename: "test".to_string(), line: 2, column: 33}),
            Token::Ident("a1".to_string(), Position{filename: "test".to_string(), line: 2, column: 35}),
            Token::Integer(123, Position{filename: "test".to_string(), line: 3, column: 1}),
            Token::Integer(1234, Position{filename: "test".to_string(), line: 4, column: 1}),
            Token::Ident("PROGRAM_SIZE".to_string(), Position{filename: "test".to_string(), line: 4, column: 6}),
            Token::Declaration("_Loop".to_string(), Position{filename: "test".to_string(), line: 5, column: 1}),
            Token::Declaration("_Read_number_".to_string(), Position{filename: "test".to_string(), line: 5, column: 8}),
            Token::Declaration("_-".to_string(), Position{filename: "test".to_string(), line: 5, column: 23}),
        ]);
    }

    #[test]
    fn tokenize_error_unknown_symbol_in_ident() {
        let text = "PROGRAM+SIZE";

        let got = tokenize(text, "test");

        assert_eq!(got.unwrap_err().to_string(), "test:1:1: failed to tokenize ident: \"PROGRAM+SIZE\"");
    }

    #[test]
    fn tokenize_error_too_big_integer() {
        let text = "123 99999999999999999999";

        let got = tokenize(text, "test");

        assert_eq!(got.unwrap_err().to_string(), "test:1:5: failed to tokenize integer: \"99999999999999999999\": number too large to fit in target type");
    }

    #[test]
    fn tokenize_error_unknown_symbol() {
        let text = "123 ~123";

        let got = tokenize(text, "test");

        assert_eq!(got.unwrap_err().to_string(), "test:1:5: token starts with illegal symbol: \"~\"");
    }

    #[test]
    fn tokenize_error_ill_formed_declaration() {
        let text = "123 :123";

        let got = tokenize(text, "test");

        assert_eq!(got.unwrap_err().to_string(), "test:1:5: failed to tokenize declaration: \":123\"");
    }

}
