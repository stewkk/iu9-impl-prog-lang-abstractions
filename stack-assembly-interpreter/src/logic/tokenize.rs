use anyhow::{anyhow, Result, Context};
use once_cell::unsync::Lazy;
use regex::Regex;

use crate::models::token::Token;

fn get_token(token_str: &str) -> Result<Token> {
    let ident_re = Lazy::new(|| Regex::new(r"^[[:alpha:]_][[:word:]-]*$").unwrap());
    let declaration_re = Lazy::new(|| Regex::new(r"^:[[:alpha:]_][[:word:]-]*$").unwrap());

    match token_str.chars().next() {
        Some('a'..='z' | 'A'..='Z' | '_') => ident_re.is_match(token_str)
                                                     .then_some(Token::Ident(token_str.to_string()))
                                                     .ok_or(anyhow!("failed to tokenize ident: {}", token_str)),
        Some('0'..='9' | '+' | '-') => token_str.parse::<i32>().map(Token::Integer)
                                                               .with_context(|| format!("failed to tokenize integer: {token_str}")),
        Some(':') => declaration_re.is_match(token_str)
                                   .then_some(Token::Declaration(token_str[1..].to_string()))
                                   .ok_or(anyhow!("failed to tokenize declaration: {}", token_str)),
        _ => Err(anyhow!("unknown token: {}", token_str))
    }
}

pub fn tokenize(text: &str) -> Result<Vec<Token>> {
    let lines = text.split('\n').filter_map(|x| x.split(';').next());
    let token_strings = lines.map(|x| x.split_whitespace()).flatten();

    token_strings.map(get_token).collect()
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

        let got = tokenize(text);

        assert_eq!(got.unwrap(), vec![
            Token::Integer(10),
            Token::Integer(65),
            Token::Integer(-40),
            Token::Declaration("_".to_string()),
            Token::Ident("_Loop".to_string()),
            Token::Declaration("a1".to_string()),
            Token::Ident("HALT".to_string()),
            Token::Ident("_Read_number_".to_string()),
            Token::Ident("_-".to_string()),
            Token::Ident("_".to_string()),
            Token::Ident("a1".to_string()),
            Token::Integer(123),
            Token::Integer(1234),
            Token::Ident("PROGRAM_SIZE".to_string()),
            Token::Declaration("_Loop".to_string()),
            Token::Declaration("_Read_number_".to_string()),
            Token::Declaration("_-".to_string()),
        ]);
    }

    #[test]
    fn tokenize_error_unknown_symbol_in_ident() {
        let text = "PROGRAM+SIZE";

        let got = tokenize(text);

        assert_eq!(got.unwrap_err().to_string(), "failed to tokenize ident: PROGRAM+SIZE");
    }

    #[test]
    fn tokenize_error_too_big_integer() {
        let text = "99999999999999999999";

        let got = tokenize(text);

        assert_eq!(got.unwrap_err().to_string(), "failed to tokenize integer: 99999999999999999999");
    }

    #[test]
    fn tokenize_error_unknown_symbol() {
        let text = "123 ~123";

        let got = tokenize(text);

        assert_eq!(got.unwrap_err().to_string(), "unknown token: ~123");
    }

    #[test]
    fn tokenize_error_ill_formed_declaration() {
        let text = "123 :123";

        let got = tokenize(text);

        assert_eq!(got.unwrap_err().to_string(), "failed to tokenize declaration: :123");
    }

}
