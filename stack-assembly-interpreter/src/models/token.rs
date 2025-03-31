#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Integer(i64, Position),
    Declaration(String, Position),
    Ident(String, Position),
}

#[derive(PartialEq, Debug, Clone)]
pub struct Position {
    pub filename: String,
    pub line: usize,
    pub column: usize,
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.filename, self.line, self.column)
    }
}
