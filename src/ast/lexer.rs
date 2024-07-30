use super::Token;

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    current_line: usize,
    current_token: Option<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            position: 0,
            current_line: 0,
            current_token: None,
        }
    }

    pub fn peek(&'a mut self) -> Option<&'a Token> {
        if self.current_token.is_none() {
            self.current_token = self.next_token();
        }

        self.current_token.as_ref()
    }

    pub fn advance(&mut self) {
        self.current_token = self.next_token();
    }

    pub fn reset(&mut self) {
        self.position = 0;
        self.current_line = 0;
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        if self.position >= self.input.len() {
            return None;
        }

        let token = match self.input.chars().nth(self.position) {
            Some(c) if c.is_ascii_alphabetic() => self.identifier(),
            Some(c) if c.is_ascii_digit() => {
                if let Some(token) = self.number().ok() {
                    token
                } else {
                    panic!("Invalid number at line {}", self.current_line)
                }
            }
            Some('"') => {
                self.position += 1;
                self.string()
                    .unwrap_or_else(|_| panic!("Unterminated string at line {}", self.current_line))
            }
            Some('+') => {
                self.position += 1;
                Token::Plus
            }
            Some('-') => {
                self.position += 1;
                Token::Minus
            }
            Some('*') => {
                self.position += 1;
                Token::Star
            }
            Some('/') => {
                self.position += 1;
                Token::Slash
            }
            Some('<') => {
                if self.input.chars().nth(self.position + 1) == Some('>') {
                    self.position += 2;
                    Token::Diamond
                } else if self.input.chars().nth(self.position + 1) == Some('=') {
                    self.position += 2;
                    Token::Le
                } else {
                    self.position += 1;
                    Token::Lt
                }
            }
            Some('>') => {
                if self.input.chars().nth(self.position + 1) == Some('=') {
                    self.position += 2;
                    Token::Ge
                } else {
                    self.position += 1;
                    Token::Gt
                }
            }
            Some('=') => {
                self.position += 1;
                Token::Eq
            }
            Some(';') => {
                self.position += 1;
                Token::Semicolon
            }
            Some(':') => {
                self.position += 1;
                Token::Colon
            }
            Some('(') => {
                self.position += 1;
                Token::LParen
            }
            Some(')') => {
                self.position += 1;
                Token::RParen
            }
            Some('\n') => {
                self.current_line += 1;

                if let Some('\r') = self.input.chars().nth(self.position + 1) {
                    self.position += 1;
                }

                self.position += 1;
                Token::Eol
            }
            Some('\r') => {
                self.current_line += 1;

                if let Some('\n') = self.input.chars().nth(self.position + 1) {
                    self.position += 1;
                }

                self.position += 1;
                Token::Eol
            }
            _ => panic!(
                "Unexpected character '{}' at line {}",
                self.input.chars().nth(self.position).unwrap(),
                self.current_line
            ),
        };

        Some(token)
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() {
            match self.input.chars().nth(self.position) {
                Some(' ' | '\t') => self.position += 1,
                _ => break,
            }
        }
    }

    fn identifier(&mut self) -> Token {
        let mut len = 0;

        for char in self.input[self.position..].chars() {
            if char.is_ascii_alphabetic() {
                len += 1;
            } else if char == '$' {
                len += 1;
                break;
            } else {
                break;
            }
        }

        let ident = &self.input[self.position..self.position + len];
        self.position += len;

        let token = match ident {
            "LET" => Token::Let,
            "GOTO" => Token::Goto,
            "GOSUB" => Token::Gosub,
            "RETURN" => Token::Return,
            "IF" => Token::If,
            "ELSE" => Token::Else,
            "THEN" => Token::Then,
            "END" => Token::End,
            "FOR" => Token::For,
            "TO" => Token::To,
            "STEP" => Token::Step,
            "NEXT" => Token::Next,
            "PRINT" => Token::Print,
            "INPUT" => Token::Input,
            "AND" => Token::And,
            "OR" => Token::Or,
            "REM" => self.comment(),
            _ => Token::Identifier(ident.to_string()),
        };

        token
    }

    fn number(&mut self) -> Result<Token, ()> {
        let mut len = 0;
        let mut chars = self.input[self.position..].chars();

        for char in chars.by_ref() {
            if char.is_ascii_digit() {
                len += 1;
            } else {
                break;
            }
        }

        let num = &self.input[self.position..self.position + len];
        self.position += len;

        Ok(Token::Number(num.parse().map_err(|_| ())?))
    }

    fn string(&mut self) -> Result<Token, ()> {
        let mut len = 0;
        let mut found_end = false;
        let mut chars = self.input[self.position..].chars();

        for char in chars.by_ref() {
            if char == '"' {
                len += 1;
                found_end = true;
                break;
            } else {
                len += 1;
            }
        }

        if !found_end {
            return Err(());
        }

        let string = &self.input[self.position..self.position + len - 1];
        self.position += len;
        Ok(Token::String(string.to_string()))
    }

    fn comment(&mut self) -> Token {
        let mut len = 0;
        let mut chars = self.input[self.position..].chars();

        for char in chars.by_ref() {
            if char == '\n' {
                break;
            } else {
                len += 1;
            }
        }

        let s = &self.input[self.position..self.position + len];
        self.position += len;

        Token::Rem(s.trim().to_string())
    }
}
