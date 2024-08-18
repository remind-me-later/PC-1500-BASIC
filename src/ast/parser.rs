use std::mem;

use super::error::ErrorKind;
use super::node::{DataItem, UnaryOperator};
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
        self.program()
    }

    fn term(&mut self) -> Result<Option<Expression>, Error> {
        match mem::take(&mut self.current_token) {
            Some(Token::Number(n)) => {
                self.current_token = self.lexer.next();
                Ok(Some(Expression::Number(n)))
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
                let res = self.expression()?;
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

    // unary + and -
    fn factor(&mut self) -> Result<Option<Expression>, Error> {
        if self.current_token == Some(Token::Plus) || self.current_token == Some(Token::Minus) {
            let op = match self.current_token {
                Some(Token::Plus) => UnaryOperator::Plus,
                Some(Token::Minus) => UnaryOperator::Minus,
                _ => unreachable!(),
            };

            self.current_token = self.lexer.next();
            let operand = self.factor();
            let operand = if let Some(operand) = operand? {
                operand
            } else {
                return Err(Error {
                    kind: ErrorKind::ExpectedExpression,
                    line: self.lexer.current_line(),
                });
            };

            Ok(Some(Expression::Unary {
                op,
                operand: Box::new(operand),
            }))
        } else {
            self.term()
        }
    }

    fn mul_div(&mut self) -> Result<Option<Expression>, Error> {
        let mut left = if let Some(left) = self.factor()? {
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
            let right = self.factor();
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

    fn add_sub(&mut self) -> Result<Option<Expression>, Error> {
        let mut left = if let Some(left) = self.mul_div()? {
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
            let right = self.mul_div();
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

    fn comparison(&mut self) -> Result<Option<Expression>, Error> {
        let mut left = if let Some(left) = self.add_sub()? {
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
            let right = self.add_sub();
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

    fn not(&mut self) -> Result<Option<Expression>, Error> {
        if self.current_token == Some(Token::Not) {
            self.current_token = self.lexer.next();
            let right = self.comparison();
            let right = if let Some(right) = right? {
                right
            } else {
                return Err(Error {
                    kind: ErrorKind::ExpectedExpression,
                    line: self.lexer.current_line(),
                });
            };

            Ok(Some(Expression::Unary {
                op: UnaryOperator::Not,
                operand: Box::new(right),
            }))
        } else {
            self.comparison()
        }
    }

    fn and(&mut self) -> Result<Option<Expression>, Error> {
        let mut left = if let Some(left) = self.not()? {
            left
        } else {
            return Ok(None);
        };

        while self.current_token == Some(Token::And) {
            self.current_token = self.lexer.next();
            let right = self.not();
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

    fn or(&mut self) -> Result<Option<Expression>, Error> {
        let mut left = if let Some(left) = self.and()? {
            left
        } else {
            return Ok(None);
        };

        while self.current_token == Some(Token::Or) {
            self.current_token = self.lexer.next();
            let right = self.and();
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

    fn expression(&mut self) -> Result<Option<Expression>, Error> {
        self.or()
    }

    fn let_(&mut self) -> Result<Statement, Error> {
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
        let expression = self.expression();
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

    fn pause(&mut self) -> Result<Statement, Error> {
        self.current_token = self.lexer.next();
        let mut content = Vec::new();

        while let Some(expr) = self.expression()? {
            content.push(expr);

            if self.current_token == Some(Token::Semicolon) {
                self.current_token = self.lexer.next();
            } else {
                break;
            }
        }

        Ok(Statement::Pause { content })
    }

    fn print(&mut self) -> Result<Statement, Error> {
        self.current_token = self.lexer.next();
        let mut content = Vec::new();

        while let Some(expr) = self.expression()? {
            content.push(expr);

            if self.current_token == Some(Token::Semicolon) {
                self.current_token = self.lexer.next();
            } else {
                break;
            }
        }

        Ok(Statement::Print { content })
    }

    fn input(&mut self) -> Result<Statement, Error> {
        self.current_token = self.lexer.next();
        let prompt = self.expression()?;

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

    fn wait(&mut self) -> Result<Statement, Error> {
        self.current_token = self.lexer.next();
        let time = self.expression()?;

        Ok(Statement::Wait { time })
    }

    fn data(&mut self) -> Result<Statement, Error> {
        self.current_token = self.lexer.next();
        let mut values = Vec::new();

        loop {
            match mem::take(&mut self.current_token) {
                Some(Token::Number(n)) => {
                    values.push(DataItem::Number(n));
                    self.current_token = self.lexer.next();
                }
                Some(Token::String(s)) => {
                    values.push(DataItem::String(s));
                    self.current_token = self.lexer.next();
                }
                _ => {
                    return Err(Error {
                        kind: ErrorKind::ExpectedDataItem,
                        line: self.lexer.current_line(),
                    });
                }
            }

            if self.current_token == Some(Token::Comma) {
                self.current_token = self.lexer.next();
            } else {
                break;
            }
        }

        Ok(Statement::Data { values })
    }

    fn read(&mut self) -> Result<Statement, Error> {
        self.current_token = self.lexer.next();
        let mut variables = Vec::new();

        loop {
            match mem::take(&mut self.current_token) {
                Some(Token::Identifier(v)) => {
                    variables.push(v);
                    self.current_token = self.lexer.next();
                }
                _ => {
                    return Err(Error {
                        kind: ErrorKind::ExpectedIdentifier,
                        line: self.lexer.current_line(),
                    });
                }
            }

            if self.current_token == Some(Token::Comma) {
                self.current_token = self.lexer.next();
            } else {
                break;
            }
        }

        Ok(Statement::Read { variables })
    }

    fn restore(&mut self) -> Result<Statement, Error> {
        self.current_token = self.lexer.next();
        let line_number = match &self.current_token {
            Some(Token::Number(n)) => match u32::try_from(*n) {
                Ok(n) => Some(n),
                Err(_) => {
                    return Err(Error {
                        kind: ErrorKind::ExpectedUnsigned,
                        line: self.lexer.current_line(),
                    });
                }
            },
            _ => None,
        };

        if line_number.is_some() {
            self.current_token = self.lexer.next();
        }

        Ok(Statement::Restore { line_number })
    }

    fn goto(&mut self) -> Result<Statement, Error> {
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

    fn return_(&mut self) -> Result<Statement, Error> {
        self.current_token = self.lexer.next();

        Ok(Statement::Return)
    }

    fn if_(&mut self) -> Result<Statement, Error> {
        self.current_token = self.lexer.next();
        let condition = match self.expression()? {
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

        let then = Box::new(self.statement()?);

        let else_ = if self.current_token == Some(Token::Else) {
            self.current_token = self.lexer.next();
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Ok(Statement::If {
            condition,
            then,
            else_,
        })
    }

    fn for_(&mut self) -> Result<Statement, Error> {
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
        let from = match self.expression()? {
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
        let to = match self.expression()? {
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
            match self.expression()? {
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

    fn next(&mut self) -> Result<Statement, Error> {
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

    fn end(&mut self) -> Result<Statement, Error> {
        self.current_token = self.lexer.next();

        Ok(Statement::End)
    }

    fn comment(&mut self) -> Result<Statement, Error> {
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

    fn atomic_statement(&mut self) -> Result<Statement, Error> {
        match self.current_token {
            Some(Token::Let | Token::Identifier(_)) => self.let_(),
            Some(Token::Print) => self.print(),
            Some(Token::Pause) => self.pause(),
            Some(Token::Input) => self.input(),
            Some(Token::Wait) => self.wait(),
            Some(Token::Goto) => self.goto(),
            Some(Token::For) => self.for_(),
            Some(Token::Next) => self.next(),
            Some(Token::End) => self.end(),
            Some(Token::Gosub) => self.parse_gosub(),
            Some(Token::If) => self.if_(),
            Some(Token::Return) => self.return_(),
            Some(Token::Data) => self.data(),
            Some(Token::Read) => self.read(),
            Some(Token::Restore) => self.restore(),
            Some(Token::Rem(_)) => self.comment(),
            _ => Err(Error {
                kind: ErrorKind::ExpectedStatement,
                line: self.lexer.current_line(),
            }),
        }
    }

    fn statement(&mut self) -> Result<Statement, Error> {
        // TODO: small vec optimization
        let mut statements = Vec::new();

        loop {
            let stmt = self.atomic_statement()?;

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

    fn line(&mut self) -> Result<(u32, Statement), Error> {
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
        let statement = self.statement()?;

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

    fn program(&mut self) -> (Program, Vec<Error>) {
        let mut errors = Vec::new();
        let mut program = Program::new();

        self.current_token = self.lexer.next();

        while self.current_token.is_some() {
            match self.line() {
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
