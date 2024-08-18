use std::{
    iter::{FusedIterator, Peekable},
    str::Chars,
};

use super::Token;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    current_line: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
            current_line: 0,
        }
    }

    pub fn current_line(&self) -> usize {
        self.current_line
    }

    fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        let token = match self.input.next()? {
            c if c.is_ascii_alphabetic() => self.identifier(c),
            c if c.is_ascii_digit() => self
                .number(c)
                .unwrap_or_else(|_| panic!("Invalid number at line {}", self.current_line)),
            '"' => self
                .string()
                .unwrap_or_else(|_| panic!("Unterminated string at line {}", self.current_line)),
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Star,
            '/' => Token::Slash,
            '<' => {
                if self.input.next_if_eq(&'>').is_some() {
                    Token::Diamond
                } else if self.input.next_if_eq(&'=').is_some() {
                    Token::LessOrEqual
                } else {
                    Token::LessThan
                }
            }
            '>' => {
                if self.input.next_if_eq(&'=').is_some() {
                    Token::GreaterOrEqual
                } else {
                    Token::GreaterThan
                }
            }
            '=' => Token::Equal,
            ',' => Token::Comma,
            ';' => Token::Semicolon,
            ':' => Token::Colon,
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '\n' | '\r' => {
                self.skip_newline();
                Token::Newline
            }
            other => panic!(
                "Unexpected character '{}' at line {}",
                other, self.current_line
            ),
        };

        Some(token)
    }

    fn skip_whitespace(&mut self) {
        while self.input.next_if(|&c| matches!(c, ' ' | '\t')).is_some() {}
    }

    // We already know the first character is a whitespace before entering this function
    fn skip_newline(&mut self) {
        while self.input.next_if(|&c| matches!(c, '\n' | '\r')).is_some() {
            self.current_line += 1;
        }
    }

    // We already know the first character is an alphabetic character before entering this function
    fn identifier(&mut self, first: char) -> Token {
        let mut ident = String::new();
        ident.push(first);

        while let Some(c) = self.input.next_if(|&c| c.is_ascii_alphabetic()) {
            ident.push(c);

            // Greedily match a keyword
            let tok = match ident.as_str() {
                "AND" => Some(Token::And),
                "CALL" => Some(Token::Call),
                "DATA" => Some(Token::Data),
                "ELSE" => Some(Token::Else),
                "END" => Some(Token::End),
                "FOR" => Some(Token::For),
                "GOSUB" => Some(Token::Gosub),
                "GOTO" => Some(Token::Goto),
                "IF" => Some(Token::If),
                "INPUT" => Some(Token::Input),
                "LET" => Some(Token::Let),
                "NEXT" => Some(Token::Next),
                "NOT" => Some(Token::Not),
                "OR" => Some(Token::Or),
                "PAUSE" => Some(Token::Pause),
                "POKE" => Some(Token::Poke),
                "PRINT" => Some(Token::Print),
                "READ" => Some(Token::Read),
                "REM" => Some(self.comment()),
                "RESTORE" => Some(Token::Restore),
                "RETURN" => Some(Token::Return),
                "STEP" => Some(Token::Step),
                "THEN" => Some(Token::Then),
                "TO" => Some(Token::To),
                "WAIT" => Some(Token::Wait),
                _ => None,
            };

            if tok.is_some() {
                return tok.unwrap();
            }
        }

        let last = self.input.peek().copied();
        if let Some('$') = last {
            ident.push('$');
            self.input.next();
        }

        Token::Identifier(ident.to_owned())
    }

    // We already know the first character is a digit before entering this function
    fn number(&mut self, first: char) -> Result<Token, ()> {
        let mut chars = String::new();
        chars.push(first);
        self.input
            .by_ref()
            .take_while(|&c| c.is_ascii_digit())
            .for_each(|c| chars.push(c));

        Ok(Token::Number(chars.parse().map_err(|_e| ())?))
    }

    // We already know the first character is a double quote before entering this function
    fn string(&mut self) -> Result<Token, ()> {
        let chars: String = self
            .input
            .by_ref()
            .take_while(|&c| c != '"' && c != '\n' && c != '\r')
            .collect();

        self.input.next(); // Consume the closing double quote, or newline

        Ok(Token::String(chars.to_owned()))
    }

    fn comment(&mut self) -> Token {
        let s: String = self
            .input
            .by_ref()
            .take_while(|&c| c != '\n' && c != '\r')
            .collect();

        Token::Rem(s.trim().to_owned())
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.input.size_hint()
    }
}

impl FusedIterator for Lexer<'_> {}
