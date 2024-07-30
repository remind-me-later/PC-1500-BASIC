pub enum ErrorKind {
    // Parse errors
    UnexpectedToken,
    MismatchedParentheses,
    ExpectedExpression,
    ExpectedStatement,
    ExpectedIdentifier,
    ExpectedUnsigned,
    ExpectedLineNumber,
    ExpectedEndOfLine,
}

pub struct Error {
    pub kind: ErrorKind,
    pub line: usize,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error at line {}: ", self.line)?;
        match self.kind {
            ErrorKind::UnexpectedToken => write!(f, "Unexpected token"),
            ErrorKind::MismatchedParentheses => write!(f, "Mismatched parentheses"),
            ErrorKind::ExpectedExpression => write!(f, "Expected expression"),
            ErrorKind::ExpectedStatement => write!(f, "Expected statement"),
            ErrorKind::ExpectedIdentifier => write!(f, "Expected identifier"),
            ErrorKind::ExpectedUnsigned => write!(f, "Expected unsigned number"),
            ErrorKind::ExpectedLineNumber => write!(f, "Expected line number"),
            ErrorKind::ExpectedEndOfLine => write!(f, "Expected end of line"),
        }
    }
}
