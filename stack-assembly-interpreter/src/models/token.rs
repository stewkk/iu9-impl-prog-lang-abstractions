#[derive(PartialEq)]
#[derive(Debug)]
pub enum Token {
    Integer(i32),
    Declaration(String),
    Ident(String),
}
