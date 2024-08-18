#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Token {
    Identifier(String),
    Number(i32),
    String(String),

    // --- Keywords ---
    Let,
    Goto,
    Gosub,
    Return,
    If,
    Else,
    Then,
    End,
    For,
    To,
    Step,
    Next,
    Dim,
    // kinda operator but treated as keyword
    And,
    Or,
    Not,
    // IO Intrinsics, might as well be keywords
    Print,
    Input,
    Pause,
    Wait,
    // Data intrinsics
    Data,
    Read,
    Restore,
    // Inline assembly
    Poke,
    Call,

    // Comments, kind of a keyword
    Rem(String),

    // --- Symbols ---
    Colon,
    Comma,
    Diamond,
    Equal,
    GreaterOrEqual,
    GreaterThan,
    LeftParen,
    LessOrEqual,
    LessThan,
    Minus,
    Newline,
    Plus,
    RightParen,
    Semicolon,
    Slash,
    Star,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            // Keywords
            Token::And => write!(f, "AND"),
            Token::Dim => write!(f, "DIM"),
            Token::Else => write!(f, "ELSE"),
            Token::End => write!(f, "END"),
            Token::For => write!(f, "FOR"),
            Token::Gosub => write!(f, "GOSUB"),
            Token::Goto => write!(f, "GOTO"),
            Token::If => write!(f, "IF"),
            Token::Let => write!(f, "LET"),
            Token::Next => write!(f, "NEXT"),
            Token::Not => write!(f, "NOT"),
            Token::Or => write!(f, "OR"),
            Token::Return => write!(f, "RETURN"),
            Token::Step => write!(f, "STEP"),
            Token::Then => write!(f, "THEN"),
            Token::To => write!(f, "TO"),
            Token::Dim => write!(f, "DIM"),
            // Intrinsics
            Token::Data => write!(f, "DATA"),
            Token::Input => write!(f, "INPUT"),
            Token::Pause => write!(f, "PAUSE"),
            Token::Print => write!(f, "PRINT"),
            Token::Read => write!(f, "READ"),
            Token::Restore => write!(f, "RESTORE"),
            Token::Wait => write!(f, "WAIT"),
            Token::Poke => write!(f, "POKE"),
            Token::Call => write!(f, "CALL"),
            // Comments
            Token::Rem(content) => write!(f, "REM({})", content),
            // Operators
            Token::Colon => write!(f, ":"),
            Token::Comma => write!(f, ","),
            Token::Diamond => write!(f, "<>"),
            Token::Equal => write!(f, "="),
            Token::GreaterOrEqual => write!(f, ">="),
            Token::GreaterThan => write!(f, ">"),
            Token::LeftParen => write!(f, "("),
            Token::LessOrEqual => write!(f, "<="),
            Token::LessThan => write!(f, "<"),
            Token::Minus => write!(f, "-"),
            Token::Newline => write!(f, "EOL"),
            Token::Plus => write!(f, "+"),
            Token::RightParen => write!(f, ")"),
            Token::Semicolon => write!(f, ";"),
            Token::Slash => write!(f, "/"),
            Token::Star => write!(f, "*"),
            // Other
            Token::Identifier(ident) => write!(f, "{}", ident),
            Token::Number(num) => write!(f, "{}", num),
            Token::String(string) => write!(f, "\"{}\"", string),
        }
    }
}
