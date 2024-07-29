use core::panic;

pub enum Token {
    // Keywords
    Let,
    Goto,
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
    // Comments
    Rem(String),
    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    // Comparison operators
    Eq,
    Diamond,
    Gt,
    Lt,
    Ge,
    Le,
    // Punctuation
    Semicolon,
    Colon,
    // Other
    Identifier(String),
    Number(i32),
    String(String),
    // Misc
    Eol,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            // Keywords
            Token::Let => write!(f, "LET"),
            Token::Goto => write!(f, "GOTO"),
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
            Token::Rem(content) => write!(f, "/*{}*/", content),
            // Operators
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Star => write!(f, "*"),
            Token::Slash => write!(f, "/"),
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
            // Other
            Token::Identifier(ident) => write!(f, "{}", ident),
            Token::Number(num) => write!(f, "{}", num),
            Token::String(string) => write!(f, "\"{}\"", string),
            // Misc
            Token::Eol => write!(f, "EOL"),
        }
    }
}

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, position: 0 }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.lex()
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() {
            match self.input.chars().nth(self.position) {
                Some(' ') | Some('\t') => self.position += 1,
                _ => break,
            }
        }
    }

    fn read_identifier(&mut self) -> Option<Token> {
        let mut len = 0;
        let mut chars = self.input[self.position..].chars();

        for char in chars.by_ref() {
            if char.is_ascii_alphabetic() {
                len += 1;
            } else {
                break;
            }
        }

        if chars.by_ref().next() == Some('$') {
            len += 1;
        }

        if len > 0 {
            let ident = &self.input[self.position..self.position + len];
            self.position += len;
            Some(Token::Identifier(ident.to_string()))
        } else {
            None
        }
    }

    fn read_number(&mut self) -> Option<Token> {
        let mut len = 0;
        let mut chars = self.input[self.position..].chars();

        for char in chars.by_ref() {
            if char.is_numeric() {
                len += 1;
            } else {
                break;
            }
        }

        if len > 0 {
            let num = &self.input[self.position..self.position + len];
            self.position += len;
            Some(Token::Number(num.parse().unwrap()))
        } else {
            None
        }
    }

    fn read_string(&mut self) -> Option<Token> {
        let mut len = 0;
        let mut chars = self.input[self.position..].chars();

        if chars.next() == Some('"') {
            len += 1;
        } else {
            return None;
        }

        for char in chars.by_ref() {
            if char == '"' {
                len += 1;
                break;
            } else {
                len += 1;
            }
        }

        if len > 0 {
            let string = &self.input[self.position + 1..self.position + len - 1];
            self.position += len;
            Some(Token::String(string.to_string()))
        } else {
            None
        }
    }

    fn read_comment(&mut self) -> Option<Token> {
        let mut len = 0;

        if self.input[self.position..].starts_with("REM") {
            len += 3;
        } else {
            return None;
        }

        let mut chars = self.input[self.position + len..].chars();

        for char in chars.by_ref() {
            if char == '\n' {
                break;
            } else {
                len += 1;
            }
        }

        let s = &self.input[self.position + 3..self.position + len];
        self.position += len;

        let s = s.trim();

        let s = s.to_string();

        Some(Token::Rem(s))
    }

    fn read_keyword(&mut self) -> Option<Token> {
        let mut len = 0;

        let keyword = if self.input[self.position..].starts_with("LET") {
            len += 3;
            Token::Let
        } else if self.input[self.position..].starts_with("GOTO") {
            len += 4;
            Token::Goto
        } else if self.input[self.position..].starts_with("IF") {
            len += 2;
            Token::If
        } else if self.input[self.position..].starts_with("ELSE") {
            len += 4;
            Token::Else
        } else if self.input[self.position..].starts_with("THEN") {
            len += 4;
            Token::Then
        } else if self.input[self.position..].starts_with("END") {
            len += 3;
            Token::End
        } else if self.input[self.position..].starts_with("FOR") {
            len += 3;
            Token::For
        } else if self.input[self.position..].starts_with("TO") {
            len += 2;
            Token::To
        } else if self.input[self.position..].starts_with("STEP") {
            len += 4;
            Token::Step
        } else if self.input[self.position..].starts_with("NEXT") {
            len += 4;
            Token::Next
        } else if self.input[self.position..].starts_with("PRINT") {
            len += 5;
            Token::Print
        } else if self.input[self.position..].starts_with("INPUT") {
            len += 5;
            Token::Input
        } else {
            return None;
        };

        self.position += len;
        Some(keyword)
    }

    fn lex_operator(&mut self) -> Option<Token> {
        let operator = match self.input.chars().nth(self.position) {
            Some('+') => Token::Plus,
            Some('-') => Token::Minus,
            Some('*') => Token::Star,
            Some('/') => Token::Slash,
            Some('<') => {
                if self.input.chars().nth(self.position + 1) == Some('>') {
                    self.position += 1;
                    Token::Diamond
                } else if self.input.chars().nth(self.position + 1) == Some('=') {
                    self.position += 1;
                    Token::Le
                } else {
                    Token::Lt
                }
            }
            Some('>') => {
                if self.input.chars().nth(self.position + 1) == Some('=') {
                    self.position += 1;
                    Token::Ge
                } else {
                    Token::Gt
                }
            }
            Some('=') => Token::Eq,
            _ => return None,
        };

        self.position += 1;
        Some(operator)
    }

    fn lex_punctuation(&mut self) -> Option<Token> {
        let punctuation = match self.input.chars().nth(self.position) {
            Some(';') => Token::Semicolon,
            Some(':') => Token::Colon,
            _ => return None,
        };

        self.position += 1;
        Some(punctuation)
    }

    fn lex_misc(&mut self) -> Option<Token> {
        let token = match self.input.chars().nth(self.position) {
            Some('\n') => Token::Eol,
            _ => return None,
        };

        self.position += 1;
        Some(token)
    }

    fn lex(&mut self) -> Option<Token> {
        self.skip_whitespace();

        if self.position >= self.input.len() {
            return None;
        }

        let token = self
            .read_comment()
            .or_else(|| self.read_keyword())
            .or_else(|| self.read_number())
            .or_else(|| self.read_string())
            .or_else(|| self.read_identifier())
            .or_else(|| self.lex_operator())
            .or_else(|| self.lex_punctuation())
            .or_else(|| self.lex_misc());

        if token.is_none() {
            panic!(
                "Unexpected character: {}",
                self.input.chars().nth(self.position).unwrap()
            );
        }

        token
    }
}
