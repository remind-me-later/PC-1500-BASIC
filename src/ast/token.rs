#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    And,
    Or,
    Eq,
    Diamond,
    Gt,
    Lt,
    Ge,
    Le,
    Semicolon,
    Colon,
    LParen,
    RParen,
    Eol,
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
            // Comparison operators
            Token::Eq => write!(f, "="),
            Token::Diamond => write!(f, "<>"),
            Token::Gt => write!(f, ">"),
            Token::Lt => write!(f, "<"),
            Token::Ge => write!(f, ">="),
            Token::Le => write!(f, "<="),
            // Punctuation
            Token::Semicolon => write!(f, ";"),
            Token::Colon => write!(f, ":"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::Eol => write!(f, "EOL"),
            // Other
            Token::Identifier(ident) => write!(f, "{}", ident),
            Token::Number(num) => write!(f, "{}", num),
            Token::String(string) => write!(f, "\"{}\"", string),
        }
    }
}
