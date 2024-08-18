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

    pub fn current_line(&self) -> usize {
        self.current_line
    }

    fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        if self.position >= self.input.len() {
            return None;
        }

        let token = match self.input.chars().nth(self.position) {
            Some(c) if c.is_ascii_alphabetic() => self.identifier(),
            Some(c) if c.is_ascii_digit() => self
                .number()
                .unwrap_or_else(|_| panic!("Invalid number at line {}", self.current_line)),
            Some('"') => self
                .string()
                .unwrap_or_else(|_| panic!("Unterminated string at line {}", self.current_line)),
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
                    Token::LessOrEqual
                } else {
                    self.position += 1;
                    Token::LessThan
                }
            }
            Some('>') => {
                if self.input.chars().nth(self.position + 1) == Some('=') {
                    self.position += 2;
                    Token::GreaterOrEqual
                } else {
                    self.position += 1;
                    Token::GreaterThan
                }
            }
            Some('=') => {
                self.position += 1;
                Token::Equal
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
                Token::LeftParen
            }
            Some(')') => {
                self.position += 1;
                Token::RightParen
            }
            Some('\n' | '\r') => {
                self.skip_newline();
                Token::Newline
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

    // We already know the first character is a whitespace before entering this function
    fn skip_newline(&mut self) {
        while self.position < self.input.len() {
            self.skip_whitespace();
            match self.input.chars().nth(self.position) {
                Some('\n' | '\r') => {
                    self.current_line += 1;
                    self.position += 1;
                }
                _ => break,
            }
        }
    }

    // We already know the first character is an alphabetic character before entering this function
    fn identifier(&mut self) -> Token {
        let mut ident = String::new();
        ident.push(self.input.chars().nth(self.position).unwrap());

        for char in self.input.get(self.position + 1..).unwrap().chars() {
            if char.is_ascii_alphabetic() {
                ident.push(char);
            } else if char == '$' {
                ident.push(char);
                break;
            } else {
                break;
            }

            // Greedily match a keyword
            let tok = match ident.as_str() {
                "LET" => Some(Token::Let),
                "GOTO" => Some(Token::Goto),
                "GOSUB" => Some(Token::Gosub),
                "RETURN" => Some(Token::Return),
                "IF" => Some(Token::If),
                "ELSE" => Some(Token::Else),
                "THEN" => Some(Token::Then),
                "END" => Some(Token::End),
                "FOR" => Some(Token::For),
                "TO" => Some(Token::To),
                "STEP" => Some(Token::Step),
                "NEXT" => Some(Token::Next),
                "PRINT" => Some(Token::Print),
                "INPUT" => Some(Token::Input),
                "AND" => Some(Token::And),
                "OR" => Some(Token::Or),
                "NOT" => Some(Token::Not),
                "REM" => Some(self.comment()),
                _ => None,
            };

            if tok.is_some() {
                self.position += ident.len();
                return tok.unwrap();
            }
        }

        self.position += ident.len();

        Token::Identifier(ident.to_owned())
    }

    // We already know the first character is a digit before entering this function
    fn number(&mut self) -> Result<Token, ()> {
        let mut len = 1;
        let chars = self.input.get(self.position + 1..).unwrap().chars();

        for char in chars {
            if char.is_ascii_digit() {
                len += 1;
            } else {
                break;
            }
        }

        let num = self.input.get(self.position..self.position + len).unwrap();
        self.position += len;

        Ok(Token::Number(num.parse().map_err(|_e| ())?))
    }

    // We already know the first character is a double quote before entering this function
    fn string(&mut self) -> Result<Token, ()> {
        let mut len = 1;
        let chars = self.input.get(self.position + 1..).unwrap().chars();

        for char in chars {
            if char == '"' || char == '\n' || char == '\r' {
                len += 1;
                break;
            } else {
                len += 1;
            }
        }

        let string = self
            .input
            .get(self.position + 1..self.position + len - 1)
            .unwrap();
        self.position += len;
        Ok(Token::String(string.to_owned()))
    }

    fn comment(&mut self) -> Token {
        let mut len = 0;
        let chars = self.input.get(self.position..).unwrap().chars();

        for char in chars {
            if char == '\n' || char == '\r' {
                break;
            } else {
                len += 1;
            }
        }

        let s = self.input.get(self.position..self.position + len).unwrap();
        self.position += len;

        Token::Rem(s.trim().to_owned())
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.input.get(self.position..).unwrap().len()))
    }
}

impl FusedIterator for Lexer<'_> {}
