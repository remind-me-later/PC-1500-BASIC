pub enum ErrorKind {
    // Parse errors
    ExpectedDataItem,
    ExpectedEndOfLine,
    ExpectedExpression,
    ExpectedIdentifier,
    ExpectedLineNumber,
    ExpectedStatement,
    ExpectedUnsigned,
    MismatchedParentheses,
    UnexpectedToken,
}

pub struct Error {
    pub kind: ErrorKind,
    pub line: usize,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error at line {}: ", self.line)?;
        match self.kind {
            ErrorKind::ExpectedDataItem => write!(f, "Expected data item"),
            ErrorKind::ExpectedEndOfLine => write!(f, "Expected end of line"),
            ErrorKind::ExpectedExpression => write!(f, "Expected expression"),
            ErrorKind::ExpectedIdentifier => write!(f, "Expected identifier"),
            ErrorKind::ExpectedLineNumber => write!(f, "Expected line number"),
            ErrorKind::ExpectedStatement => write!(f, "Expected statement"),
            ErrorKind::ExpectedUnsigned => write!(f, "Expected unsigned number"),
            ErrorKind::MismatchedParentheses => write!(f, "Mismatched parentheses"),
            ErrorKind::UnexpectedToken => write!(f, "Unexpected token"),
        }
    }
}
