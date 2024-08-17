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
    // kinda operator but treated as keyword
    And,
    Or,
    Not,
    // Intrinsics, might as well be keywords
    Print,
    Input,
    // Comments, kind of a keyword
    Rem(String),

    // --- Symbols ---
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    Diamond,
    GreaterThan,
    LessThan,
    GreaterOrEqual,
    LessOrEqual,
    Semicolon,
    Colon,
    LeftParen,
    RightParen,
    Newline,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            // Keywords
            Token::Let => write!(f, "LET"),
            Token::Goto => write!(f, "GOTO"),
            Token::Gosub => write!(f, "GOSUB"),
            Token::Return => write!(f, "RETURN"),
            Token::If => write!(f, "IF"),
            Token::Else => write!(f, "ELSE"),
            Token::Then => write!(f, "THEN"),
            Token::End => write!(f, "END"),
            Token::For => write!(f, "FOR"),
            Token::To => write!(f, "TO"),
            Token::Step => write!(f, "STEP"),
            Token::Next => write!(f, "NEXT"),
            // Intrinsics
            Token::Print => write!(f, "PRINT"),
            Token::Input => write!(f, "INPUT"),
            // Comments
            Token::Rem(content) => write!(f, "REM({})", content),
            // Operators
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Star => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::And => write!(f, "AND"),
            Token::Or => write!(f, "OR"),
            Token::Not => write!(f, "NOT"),
            // Comparison operators
            Token::Equal => write!(f, "="),
            Token::Diamond => write!(f, "<>"),
            Token::GreaterThan => write!(f, ">"),
            Token::LessThan => write!(f, "<"),
            Token::GreaterOrEqual => write!(f, ">="),
            Token::LessOrEqual => write!(f, "<="),
            // Punctuation
            Token::Semicolon => write!(f, ";"),
            Token::Colon => write!(f, ":"),
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
            Token::Newline => write!(f, "EOL"),
            // Other
            Token::Identifier(ident) => write!(f, "{}", ident),
            Token::Number(num) => write!(f, "{}", num),
            Token::String(string) => write!(f, "\"{}\"", string),
        }
    }
}
