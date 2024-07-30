pub enum ErrorKind {
    UnexpectedToken,
    UnexpectedEof,
    InvalidNumber,
    UnterminatedString,
    MismatchedParentheses,
    ExpectedExpression,
    ExpectedStatement,
    ExpectedIdentifier,
    ExpectedUnsigned,
    ExpectedLineNumber,
}

pub struct Error {
    pub kind: ErrorKind,
    pub line: usize,
}
