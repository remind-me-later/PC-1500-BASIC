use std::mem;

use super::error::ErrorKind;
use super::{BinaryOperator, Error, Expression, Program, Statement};
use super::{Lexer, Token};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self {
            lexer,
            current_token: None,
        }
    }

    pub fn parse(&mut self) -> (Program, Vec<Error>) {
        self.parse_program()
    }

    fn parse_factor(&mut self) -> Result<Option<Expression>, Error> {
        match mem::take(&mut self.current_token) {
            Some(Token::Number(n)) => {
                self.current_token = self.lexer.next();
                Ok(Some(Expression::NumberLiteral(n)))
            }
            Some(Token::Identifier(v)) => {
                self.current_token = self.lexer.next();
                Ok(Some(Expression::Variable(v)))
            }
            Some(Token::String(s)) => {
                self.current_token = self.lexer.next();
                Ok(Some(Expression::StringLiteral(s)))
            }
            Some(Token::LeftParen) => {
                self.current_token = self.lexer.next();
                let res = self.parse_expression()?;
                if self.current_token == Some(Token::RightParen) {
                    self.current_token = self.lexer.next();
                    Ok(res)
                } else {
                    Err(Error {
                        kind: ErrorKind::MismatchedParentheses,
                        line: self.lexer.current_line(),
                    })
                }
            }
            other => {
                self.current_token = other;
                Ok(None)
            }
        }
    }

    fn parse_mul_div(&mut self) -> Result<Option<Expression>, Error> {
        let mut left = if let Some(left) = self.parse_factor()? {
            left
        } else {
            return Ok(None);
        };

        while let Some(Token::Star) | Some(Token::Slash) = self.current_token {
            let op = match self.current_token {
                Some(Token::Star) => BinaryOperator::Mul,
                Some(Token::Slash) => BinaryOperator::Div,
                _ => unreachable!(),
            };

            self.current_token = self.lexer.next();
            let right = self.parse_factor();
            let right = if let Some(right) = right? {
                right
            } else {
                return Err(Error {
                    kind: ErrorKind::ExpectedExpression,
                    line: self.lexer.current_line(),
                });
            };

            left = Expression::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(Some(left))
    }

    fn parse_add_sub(&mut self) -> Result<Option<Expression>, Error> {
        let mut left = if let Some(left) = self.parse_mul_div()? {
            left
        } else {
            return Ok(None);
        };

        while let Some(Token::Plus) | Some(Token::Minus) = self.current_token {
            let op = match self.current_token {
                Some(Token::Plus) => BinaryOperator::Add,
                Some(Token::Minus) => BinaryOperator::Sub,
                _ => unreachable!(),
            };

            self.current_token = self.lexer.next();
            let right = self.parse_mul_div();
            let right = if let Some(right) = right? {
                right
            } else {
                return Err(Error {
                    kind: ErrorKind::ExpectedExpression,
                    line: self.lexer.current_line(),
                });
            };

            left = Expression::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(Some(left))
    }

    fn parse_comparison(&mut self) -> Result<Option<Expression>, Error> {
        let mut left = if let Some(left) = self.parse_add_sub()? {
            left
        } else {
            return Ok(None);
        };

        while let Some(Token::Equal)
        | Some(Token::Diamond)
        | Some(Token::LessThan)
        | Some(Token::LessOrEqual)
        | Some(Token::GreaterThan)
        | Some(Token::GreaterOrEqual) = self.current_token
        {
            let op = match self.current_token {
                Some(Token::Equal) => BinaryOperator::Eq,
                Some(Token::Diamond) => BinaryOperator::Ne,
                Some(Token::LessThan) => BinaryOperator::Lt,
                Some(Token::LessOrEqual) => BinaryOperator::Le,
                Some(Token::GreaterThan) => BinaryOperator::Gt,
                Some(Token::GreaterOrEqual) => BinaryOperator::Ge,
                _ => unreachable!(),
            };

            self.current_token = self.lexer.next();
            let right = self.parse_add_sub();
            let right = if let Some(right) = right? {
                right
            } else {
                return Err(Error {
                    kind: ErrorKind::ExpectedExpression,
                    line: self.lexer.current_line(),
                });
            };

            left = Expression::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(Some(left))
    }

    fn parse_and(&mut self) -> Result<Option<Expression>, Error> {
        let mut left = if let Some(left) = self.parse_comparison()? {
            left
        } else {
            return Ok(None);
        };

        while self.current_token == Some(Token::And) {
            self.current_token = self.lexer.next();
            let right = self.parse_comparison();
            let right = if let Some(right) = right? {
                right
            } else {
                return Err(Error {
                    kind: ErrorKind::ExpectedExpression,
                    line: self.lexer.current_line(),
                });
            };

            left = Expression::Binary {
                left: Box::new(left),
                op: BinaryOperator::And,
                right: Box::new(right),
            };
        }

        Ok(Some(left))
    }

    fn parse_or(&mut self) -> Result<Option<Expression>, Error> {
        let mut left = if let Some(left) = self.parse_and()? {
            left
        } else {
            return Ok(None);
        };

        while self.current_token == Some(Token::Or) {
            self.current_token = self.lexer.next();
            let right = self.parse_and();
            let right = if let Some(right) = right? {
                right
            } else {
                return Err(Error {
                    kind: ErrorKind::ExpectedExpression,
                    line: self.lexer.current_line(),
                });
            };

            left = Expression::Binary {
                left: Box::new(left),
                op: BinaryOperator::Or,
                right: Box::new(right),
            };
        }

        Ok(Some(left))
    }

    fn parse_expression(&mut self) -> Result<Option<Expression>, Error> {
        self.parse_or()
    }

    fn parse_let(&mut self) -> Result<Statement, Error> {
        let variable = match mem::take(&mut self.current_token) {
            // Optional LET keyword
            Some(Token::Let) => {
                self.current_token = self.lexer.next();

                match mem::take(&mut self.current_token) {
                    Some(Token::Identifier(v)) => v,
                    _ => {
                        return Err(Error {
                            kind: ErrorKind::ExpectedIdentifier,
                            line: self.lexer.current_line(),
                        });
                    }
                }
            }
            Some(Token::Identifier(v)) => v,
            _ => {
                unreachable!("We already checked for LET or identifier");
            }
        };

        self.current_token = self.lexer.next();
        if self.current_token != Some(Token::Equal) {
            return Err(Error {
                kind: ErrorKind::UnexpectedToken,
                line: self.lexer.current_line(),
            });
        }

        self.current_token = self.lexer.next();
        let expression = self.parse_expression();
        let expression = if let Some(expression) = expression? {
            expression
        } else {
            return Err(Error {
                kind: ErrorKind::ExpectedExpression,
                line: self.lexer.current_line(),
            });
        };

        Ok(Statement::Let {
            variable,
            expression,
        })
    }

    fn parse_print(&mut self) -> Result<Statement, Error> {
        self.current_token = self.lexer.next();
        let mut content = Vec::new();

        while let Some(expr) = self.parse_expression()? {
            content.push(expr);

            if self.current_token == Some(Token::Semicolon) {
                self.current_token = self.lexer.next();
            } else {
                break;
            }
        }

        Ok(Statement::Print { content })
    }

    fn parse_input(&mut self) -> Result<Statement, Error> {
        self.current_token = self.lexer.next();
        let prompt = self.parse_expression()?;

        if self.current_token == Some(Token::Semicolon) {
            self.current_token = self.lexer.next();
        }

        let variable = match mem::take(&mut self.current_token) {
            Some(Token::Identifier(v)) => v,
            _ => {
                return Err(Error {
                    kind: ErrorKind::ExpectedIdentifier,
                    line: self.lexer.current_line(),
                });
            }
        };

        self.current_token = self.lexer.next();

        Ok(Statement::Input { prompt, variable })
    }

    fn parse_goto(&mut self) -> Result<Statement, Error> {
        self.current_token = self.lexer.next();
        let line_number = match &self.current_token {
            Some(Token::Number(n)) => match u32::try_from(*n) {
                Ok(n) => n,
                Err(_) => {
                    return Err(Error {
                        kind: ErrorKind::ExpectedUnsigned,
                        line: self.lexer.current_line(),
                    });
                }
            },
            _ => {
                return Err(Error {
                    kind: ErrorKind::ExpectedUnsigned,
                    line: self.lexer.current_line(),
                });
            }
        };

        self.current_token = self.lexer.next();

        Ok(Statement::Goto { line_number })
    }

    fn parse_gosub(&mut self) -> Result<Statement, Error> {
        self.current_token = self.lexer.next();
        let line_number = match &self.current_token {
            Some(Token::Number(n)) => match u32::try_from(*n) {
                Ok(n) => n,
                Err(_) => {
                    return Err(Error {
                        kind: ErrorKind::ExpectedUnsigned,
                        line: self.lexer.current_line(),
                    });
                }
            },
            _ => {
                return Err(Error {
                    kind: ErrorKind::ExpectedUnsigned,
                    line: self.lexer.current_line(),
                });
            }
        };

        self.current_token = self.lexer.next();

        Ok(Statement::GoSub { line_number })
    }

    fn parse_return(&mut self) -> Result<Statement, Error> {
        self.current_token = self.lexer.next();

        Ok(Statement::Return)
    }

    fn parse_if(&mut self) -> Result<Statement, Error> {
        self.current_token = self.lexer.next();
        let condition = match self.parse_expression()? {
            Some(expr) => expr,
            None => {
                return Err(Error {
                    kind: ErrorKind::ExpectedExpression,
                    line: self.lexer.current_line(),
                });
            }
        };

        if self.current_token == Some(Token::Then) {
            self.current_token = self.lexer.next();
        }

        let then = Box::new(self.parse_statement()?);

        let else_ = if self.current_token == Some(Token::Else) {
            self.current_token = self.lexer.next();
            Some(Box::new(self.parse_statement()?))
        } else {
            None
        };

        Ok(Statement::If {
            condition,
            then,
            else_,
        })
    }

    fn parse_for(&mut self) -> Result<Statement, Error> {
        self.current_token = self.lexer.next();
        let variable = match mem::take(&mut self.current_token) {
            Some(Token::Identifier(v)) => v,
            _ => {
                return Err(Error {
                    kind: ErrorKind::ExpectedIdentifier,
                    line: self.lexer.current_line(),
                });
            }
        };

        self.current_token = self.lexer.next();
        if self.current_token != Some(Token::Equal) {
            return Err(Error {
                kind: ErrorKind::UnexpectedToken,
                line: self.lexer.current_line(),
            });
        }

        self.current_token = self.lexer.next();
        let from = match self.parse_expression()? {
            Some(expr) => expr,
            None => {
                return Err(Error {
                    kind: ErrorKind::ExpectedExpression,
                    line: self.lexer.current_line(),
                });
            }
        };

        if self.current_token != Some(Token::To) {
            return Err(Error {
                kind: ErrorKind::UnexpectedToken,
                line: self.lexer.current_line(),
            });
        }

        self.current_token = self.lexer.next();
        let to = match self.parse_expression()? {
            Some(expr) => expr,
            None => {
                return Err(Error {
                    kind: ErrorKind::ExpectedExpression,
                    line: self.lexer.current_line(),
                });
            }
        };

        let step = if self.current_token == Some(Token::Step) {
            self.current_token = self.lexer.next();
            match self.parse_expression()? {
                Some(expr) => Some(expr),
                None => {
                    return Err(Error {
                        kind: ErrorKind::ExpectedExpression,
                        line: self.lexer.current_line(),
                    });
                }
            }
        } else {
            None
        };

        Ok(Statement::For {
            variable,
            from,
            to,
            step,
        })
    }

    fn parse_next(&mut self) -> Result<Statement, Error> {
        self.current_token = self.lexer.next();
        let variable = match mem::take(&mut self.current_token) {
            Some(Token::Identifier(v)) => v,
            _ => {
                return Err(Error {
                    kind: ErrorKind::ExpectedIdentifier,
                    line: self.lexer.current_line(),
                });
            }
        };

        self.current_token = self.lexer.next();

        Ok(Statement::Next { variable })
    }

    fn parse_end(&mut self) -> Result<Statement, Error> {
        self.current_token = self.lexer.next();

        Ok(Statement::End)
    }

    fn parse_atomic_statement(&mut self) -> Result<Statement, Error> {
        match self.current_token {
            Some(Token::Let | Token::Identifier(_)) => self.parse_let(),
            Some(Token::Print) => self.parse_print(),
            Some(Token::Input) => self.parse_input(),
            Some(Token::Goto) => self.parse_goto(),
            Some(Token::For) => self.parse_for(),
            Some(Token::Next) => self.parse_next(),
            Some(Token::End) => self.parse_end(),
            Some(Token::Gosub) => self.parse_gosub(),
            Some(Token::If) => self.parse_if(),
            Some(Token::Return) => self.parse_return(),
            Some(Token::Rem(_)) => self.parse_comment(),
            _ => Err(Error {
                kind: ErrorKind::ExpectedStatement,
                line: self.lexer.current_line(),
            }),
        }
    }

    fn parse_statement(&mut self) -> Result<Statement, Error> {
        // TODO: small vec optimization
        let mut statements = Vec::new();

        loop {
            let stmt = self.parse_atomic_statement()?;

            statements.push(stmt);

            if self.current_token == Some(Token::Colon) {
                self.current_token = self.lexer.next();
            } else {
                break;
            }
        }

        Ok(if statements.len() == 1 {
            statements.remove(0)
        } else {
            Statement::Seq { statements }
        })
    }

    fn parse_comment(&mut self) -> Result<Statement, Error> {
        match mem::take(&mut self.current_token) {
            Some(Token::Rem(s)) => {
                self.current_token = self.lexer.next();
                Ok(Statement::Rem { content: s })
            }
            _ => {
                unreachable!("We already checked for REM");
            }
        }
    }

    fn parse_line(&mut self) -> Result<(u32, Statement), Error> {
        let line_number = match &self.current_token {
            Some(Token::Number(n)) => {
                if let Ok(n) = u32::try_from(*n) {
                    n
                } else {
                    return Err(Error {
                        kind: ErrorKind::ExpectedLineNumber,
                        line: self.lexer.current_line(),
                    });
                }
            }
            _ => {
                return Err(Error {
                    kind: ErrorKind::ExpectedLineNumber,
                    line: self.lexer.current_line(),
                })
            }
        };

        self.current_token = self.lexer.next();
        let statement = self.parse_statement()?;

        match self.current_token {
            Some(Token::Newline) => {
                self.current_token = self.lexer.next();
            }
            None => {}
            _ => {
                return Err(Error {
                    kind: ErrorKind::ExpectedEndOfLine,
                    line: self.lexer.current_line(),
                });
            }
        }

        Ok((line_number, statement))
    }

    fn parse_program(&mut self) -> (Program, Vec<Error>) {
        let mut errors = Vec::new();
        let mut program = Program::new();

        self.current_token = self.lexer.next();

        while self.current_token.is_some() {
            match self.parse_line() {
                Ok((line_number, statement)) => {
                    program.add_line(line_number, statement);
                }
                Err(e) => {
                    errors.push(e);
                    self.current_token = self.lexer.next();

                    while self.current_token != Some(Token::Newline) {
                        self.current_token = self.lexer.next();
                    }
                }
            }
        }

        (program, errors)
    }
}
