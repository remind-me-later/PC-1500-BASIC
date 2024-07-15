#[derive(Debug, PartialEq, Clone)]
pub enum Tok {
    // Keywords
    Let,
    For,
    To,
    Step,
    Next,
    Input,
    Print,
    End,
    Goto,
    Gosub,
    Return,
    If,
    Then,
    Else,
    // comments
    Rem(String),
    // Identifiers
    Identifier(String),
    // Literals
    Number(u64),
    StringLiteral(String),
    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Eq,
    LParen,
    RParen,
    Colon,
    SemiColon,
    Diamond,
    GreaterThan,
    LessThan,
    And,
    Or,
    // Misc
    Eol,
    Eof,
}

impl std::fmt::Display for Tok {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tok::Let => write!(f, "LET"),
            Tok::For => write!(f, "FOR"),
            Tok::To => write!(f, "TO"),
            Tok::Step => write!(f, "STEP"),
            Tok::Next => write!(f, "NEXT"),
            Tok::Input => write!(f, "INPUT"),
            Tok::Print => write!(f, "PRINT"),
            Tok::End => write!(f, "END"),
            Tok::Goto => write!(f, "GOTO"),
            Tok::Gosub => write!(f, "GOSUB"),
            Tok::Return => write!(f, "RETURN"),
            Tok::If => write!(f, "IF"),
            Tok::Then => write!(f, "THEN"),
            Tok::Else => write!(f, "ELSE"),
            Tok::Rem(comment) => write!(f, "REM {}", comment),
            Tok::Identifier(id) => write!(f, "{}", id),
            Tok::Number(num) => write!(f, "{}", num),
            Tok::StringLiteral(s) => write!(f, "\"{}\"", s),
            Tok::Plus => write!(f, "+"),
            Tok::Minus => write!(f, "-"),
            Tok::Star => write!(f, "*"),
            Tok::Slash => write!(f, "/"),
            Tok::Eq => write!(f, "="),
            Tok::LParen => write!(f, "("),
            Tok::RParen => write!(f, ")"),
            Tok::Colon => write!(f, ":"),
            Tok::SemiColon => write!(f, ";"),
            Tok::Diamond => write!(f, "<>"),
            Tok::GreaterThan => write!(f, ">"),
            Tok::LessThan => write!(f, "<"),
            Tok::And => write!(f, "AND"),
            Tok::Or => write!(f, "OR"),
            Tok::Eol => writeln!(f),
            Tok::Eof => write!(f, "EOF"),
        }
    }
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input: input.chars().collect(),
            position: 0,
            current_char: None,
        };
        lexer.current_char = lexer.input.get(lexer.position).cloned();
        lexer
    }

    pub fn next_token(&mut self) -> Tok {
        self.skip_whitespace();
        if let Some(ch) = self.current_char {
            let token = match ch {
                '+' => Tok::Plus,
                '-' => Tok::Minus,
                '*' => Tok::Star,
                '/' => Tok::Slash,
                '=' => Tok::Eq,
                '(' => Tok::LParen,
                ')' => Tok::RParen,
                ':' => Tok::Colon,
                ';' => Tok::SemiColon,
                '<' => {
                    self.advance();
                    if self.current_char == Some('>') {
                        self.advance();
                        Tok::Diamond
                    } else {
                        Tok::LessThan
                    }
                }
                '>' => Tok::GreaterThan,
                '\n' => Tok::Eol,
                '0'..='9' | '.' => return self.number(),
                'A'..='Z' | 'a'..='z' => return self.identifier(),
                '"' => return self.string_literal(),
                _ => Tok::Eof,
            };
            self.advance();
            token
        } else {
            Tok::Eof
        }
    }

    fn advance(&mut self) {
        self.position += 1;
        self.current_char = self.input.get(self.position).cloned();
    }

    fn skip_whitespace(&mut self) {
        while self.current_char == Some(' ') {
            self.advance();
        }
    }

    fn number(&mut self) -> Tok {
        let mut num_str = String::new();
        while let Some(ch) = self.current_char {
            if ch.is_ascii_digit() {
                num_str.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        Tok::Number(num_str.parse().unwrap())
    }

    fn string_literal(&mut self) -> Tok {
        let mut str_lit = String::new();
        self.advance();
        while let Some(ch) = self.current_char {
            if ch == '"' {
                self.advance();
                break;
            } else {
                str_lit.push(ch);
                self.advance();
            }
        }
        Tok::StringLiteral(str_lit)
    }

    fn rem(&mut self) -> Tok {
        let mut comment = String::new();
        while let Some(ch) = self.current_char {
            if ch == '\n' {
                break;
            }
            comment.push(ch);
            self.advance();
        }
        Tok::Rem(comment)
    }

    fn identifier(&mut self) -> Tok {
        let mut id_str = String::new();

        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || ch == '$' {
                id_str.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        match id_str.as_str() {
            "LET" => Tok::Let,
            "FOR" => Tok::For,
            "TO" => Tok::To,
            "STEP" => Tok::Step,
            "NEXT" => Tok::Next,
            "INPUT" => Tok::Input,
            "PRINT" => Tok::Print,
            "END" => Tok::End,
            "GOTO" => Tok::Goto,
            "GOSUB" => Tok::Gosub,
            "RETURN" => Tok::Return,
            "IF" => Tok::If,
            "THEN" => Tok::Then,
            "ELSE" => Tok::Else,
            "AND" => Tok::And,
            "OR" => Tok::Or,
            // Special case for REM (comments)
            "REM" => {
                self.skip_whitespace();
                self.rem()
            }
            _ => Tok::Identifier(id_str),
        }
    }
}
