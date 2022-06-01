#[derive(Debug)]
pub enum Error {
    Syntax(String),
    InvalidOp(String),
    UndefinedVar(String),
    Internal(String),
    StackOverflow,
}
