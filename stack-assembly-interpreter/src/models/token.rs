#[derive(PartialEq)]
#[derive(Debug)]
pub enum Token {
    Integer(i32, Position),
    Declaration(String, Position),
    Ident(String, Position),
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Position {
    pub filename: String,
    pub line: usize,
    pub column: usize,
}
