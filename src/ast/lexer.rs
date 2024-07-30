use std::iter::FusedIterator;

use super::Token;

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    current_line: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            position: 0,
            current_line: 0,
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        if self.position >= self.input.len() {
            return None;
        }

        let token = match self.input.bytes().nth(self.position) {
            Some(c) if c.is_ascii_alphabetic() => self.identifier(),
            Some(c) if c.is_ascii_digit() => {
                if let Some(token) = self.number().ok() {
                    token
                } else {
                    panic!("Invalid number at line {}", self.current_line)
                }
            }
            Some(b'"') => {
                self.position += 1;
                self.string()
                    .unwrap_or_else(|_| panic!("Unterminated string at line {}", self.current_line))
            }
            Some(b'+') => {
                self.position += 1;
                Token::Plus
            }
            Some(b'-') => {
                self.position += 1;
                Token::Minus
            }
            Some(b'*') => {
                self.position += 1;
                Token::Star
            }
            Some(b'/') => {
                self.position += 1;
                Token::Slash
            }
            Some(b'<') => {
                if self.input.bytes().nth(self.position + 1) == Some(b'>') {
                    self.position += 2;
                    Token::Diamond
                } else if self.input.bytes().nth(self.position + 1) == Some(b'=') {
                    self.position += 2;
                    Token::LessOrEqual
                } else {
                    self.position += 1;
                    Token::LessThan
                }
            }
            Some(b'>') => {
                if self.input.bytes().nth(self.position + 1) == Some(b'=') {
                    self.position += 2;
                    Token::GreaterOrEqual
                } else {
                    self.position += 1;
                    Token::GreaterThan
                }
            }
            Some(b'=') => {
                self.position += 1;
                Token::Equal
            }
            Some(b';') => {
                self.position += 1;
                Token::Semicolon
            }
            Some(b':') => {
                self.position += 1;
                Token::Colon
            }
            Some(b'(') => {
                self.position += 1;
                Token::LeftParen
            }
            Some(b')') => {
                self.position += 1;
                Token::RightParen
            }
            Some(b'\n' | b'\r') => {
                self.skip_newline();
                Token::Newline
            }
            _ => panic!(
                "Unexpected character '{}' at line {}",
                self.input.bytes().nth(self.position).unwrap(),
                self.current_line
            ),
        };

        Some(token)
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() {
            match self.input.bytes().nth(self.position) {
                Some(b' ' | b'\t') => self.position += 1,
                _ => break,
            }
        }
    }

    fn skip_newline(&mut self) {
        while self.position < self.input.len() {
            self.skip_whitespace();
            match self.input.bytes().nth(self.position) {
                Some(b'\n' | b'\r') => {
                    self.current_line += 1;
                    self.position += 1;
                }
                _ => break,
            }
        }
    }

    // We already know the first character is an alphabetic character before entering this function
    fn identifier(&mut self) -> Token {
        let mut len = 1;

        for char in self.input[self.position + 1..].bytes() {
            if char.is_ascii_alphabetic() {
                len += 1;
            } else if char == b'$' {
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

    // We already know the first character is a digit before entering this function
    fn number(&mut self) -> Result<Token, ()> {
        let mut len = 1;
        let mut chars = self.input[self.position + 1..].bytes();

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
        let mut chars = self.input[self.position..].bytes();

        for char in chars.by_ref() {
            if char == b'"' {
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
        let mut chars = self.input[self.position..].bytes();

        for char in chars.by_ref() {
            if char == b'\n' || char == b'\r' {
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

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.input[self.position..].len()))
    }
}

impl FusedIterator for Lexer<'_> {}
