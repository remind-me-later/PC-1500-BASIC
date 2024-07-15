#[derive(Debug, PartialEq, Clone)]
pub enum Tok {
    // Keywords
    Let,
    For,
    To,
    Step,
    Next,
    Print,
    End,
    Goto,
    Gosub,
    Return,
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
    Assign,
    LParen,
    RParen,
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
            Tok::Print => write!(f, "PRINT"),
            Tok::End => write!(f, "END"),
            Tok::Goto => write!(f, "GOTO"),
            Tok::Gosub => write!(f, "GOSUB"),
            Tok::Return => write!(f, "RETURN"),
            Tok::Rem(comment) => write!(f, "REM {}", comment),
            Tok::Identifier(id) => write!(f, "{}", id),
            Tok::Number(num) => write!(f, "{}", num),
            Tok::StringLiteral(s) => write!(f, "\"{}\"", s),
            Tok::Plus => write!(f, "+"),
            Tok::Minus => write!(f, "-"),
            Tok::Star => write!(f, "*"),
            Tok::Slash => write!(f, "/"),
            Tok::Assign => write!(f, "="),
            Tok::LParen => write!(f, "("),
            Tok::RParen => write!(f, ")"),
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
                '=' => Tok::Assign,
                '(' => Tok::LParen,
                ')' => Tok::RParen,
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
            if ch.is_alphanumeric() {
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
            "PRINT" => Tok::Print,
            "END" => Tok::End,
            "GOTO" => Tok::Goto,
            "GOSUB" => Tok::Gosub,
            "RETURN" => Tok::Return,
            // Special case for REM (comments)
            "REM" => {
                self.skip_whitespace();
                self.rem()
            }
            _ => Tok::Identifier(id_str),
        }
    }
}
