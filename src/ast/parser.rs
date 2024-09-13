use std::iter::Peekable;
use std::mem;

use super::error::ErrorKind;
use super::node::{DataItem, LValue, UnaryOperator};
use super::{BinaryOperator, Error, Expression, Program, Statement};
use crate::tokens::{Lexer, Token};

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self {
            lexer: lexer.peekable(),
        }
    }

    pub fn parse(&mut self) -> (Program, Vec<Error>) {
        // self.program()
        todo!("parse")
    }

    // fn let_(&mut self) -> Result<Statement, Error> {
    //     // println!("let");
    //     let variable = match &mut self.current_token {
    //         // Optional LET keyword
    //         Some(Token::Let) => {
    //             self.current_token = self.lexer.next();

    //             match self.current_token {
    //                 Some(Token::Identifier(_)) => self.lvalue()?,
    //                 _ => {
    //                     return Err(Error {
    //                         kind: ErrorKind::ExpectedIdentifier,
    //                         line: self.lexer.current_line(),
    //                     });
    //                 }
    //             }
    //         }
    //         Some(Token::Identifier(v)) => {
    //             self.current_token = Some(Token::Identifier(mem::take(v)));
    //             println!("identifier");
    //             self.lvalue()?
    //         }
    //         _ => {
    //             unreachable!("We already checked for LET or identifier");
    //         }
    //     };

    //     // println!("variable: {:?}", variable);
    //     // println!("current_token: {:?}", self.current_token);

    //     if self.current_token != Some(Token::Equal) {
    //         // println!("not equal {:?}", self.current_token);
    //         return Err(Error {
    //             kind: ErrorKind::UnexpectedToken,
    //             line: self.lexer.current_line(),
    //         });
    //     }

    //     println!("equal");

    //     self.current_token = self.lexer.next();
    //     let expression = self.expression()?;
    //     let expression = if let Some(expression) = expression {
    //         println!("expression");
    //         expression
    //     } else {
    //         println!("no expression");
    //         return Err(Error {
    //             kind: ErrorKind::ExpectedExpression,
    //             line: self.lexer.current_line(),
    //         });
    //     };

    //     Ok(Statement::Let {
    //         variable,
    //         expression,
    //     })
    // }

    // fn pause(&mut self) -> Result<Statement, Error> {
    //     self.current_token = self.lexer.next();
    //     let mut content = Vec::new();

    //     while let Some(expr) = self.expression()? {
    //         content.push(expr);

    //         if self.current_token == Some(Token::Semicolon) {
    //             self.current_token = self.lexer.next();
    //         } else {
    //             break;
    //         }
    //     }

    //     Ok(Statement::Pause { content })
    // }

    // fn print(&mut self) -> Result<Statement, Error> {
    //     self.current_token = self.lexer.next();
    //     let mut content = Vec::new();

    //     while let Some(expr) = self.expression()? {
    //         content.push(expr);

    //         if self.current_token == Some(Token::Semicolon) {
    //             self.current_token = self.lexer.next();
    //         } else {
    //             break;
    //         }
    //     }

    //     Ok(Statement::Print { content })
    // }

    // fn input(&mut self) -> Result<Statement, Error> {
    //     self.current_token = self.lexer.next();
    //     let prompt = self.expression()?;

    //     if self.current_token == Some(Token::Semicolon) {
    //         self.current_token = self.lexer.next();
    //     }

    //     let variable = match self.current_token {
    //         Some(Token::Identifier(_)) => self.lvalue()?,
    //         _ => {
    //             return Err(Error {
    //                 kind: ErrorKind::ExpectedIdentifier,
    //                 line: self.lexer.current_line(),
    //             });
    //         }
    //     };

    //     self.current_token = self.lexer.next();

    //     Ok(Statement::Input { prompt, variable })
    // }

    // fn wait(&mut self) -> Result<Statement, Error> {
    //     self.current_token = self.lexer.next();
    //     let time = self.expression()?;

    //     Ok(Statement::Wait { time })
    // }

    // fn data(&mut self) -> Result<Statement, Error> {
    //     self.current_token = self.lexer.next();
    //     let mut values = Vec::new();

    //     loop {
    //         match &mut self.current_token {
    //             Some(Token::Number(n)) => {
    //                 values.push(DataItem::Number(*n));
    //                 self.current_token = self.lexer.next();
    //             }
    //             Some(Token::String(s)) => {
    //                 values.push(DataItem::String(std::mem::take(s)));
    //                 self.current_token = self.lexer.next();
    //             }
    //             _ => {
    //                 return Err(Error {
    //                     kind: ErrorKind::ExpectedDataItem,
    //                     line: self.lexer.current_line(),
    //                 });
    //             }
    //         }

    //         if self.current_token == Some(Token::Comma) {
    //             self.current_token = self.lexer.next();
    //         } else {
    //             break;
    //         }
    //     }

    //     Ok(Statement::Data { values })
    // }

    // fn read(&mut self) -> Result<Statement, Error> {
    //     self.current_token = self.lexer.next();
    //     let mut variables = Vec::new();

    //     loop {
    //         match self.current_token {
    //             Some(Token::Identifier(_)) => {
    //                 variables.push(self.lvalue()?);
    //                 self.current_token = self.lexer.next();
    //             }
    //             _ => {
    //                 return Err(Error {
    //                     kind: ErrorKind::ExpectedIdentifier,
    //                     line: self.lexer.current_line(),
    //                 });
    //             }
    //         }

    //         if self.current_token == Some(Token::Comma) {
    //             self.current_token = self.lexer.next();
    //         } else {
    //             break;
    //         }
    //     }

    //     Ok(Statement::Read { variables })
    // }

    // fn restore(&mut self) -> Result<Statement, Error> {
    //     self.current_token = self.lexer.next();
    //     let line_number = match &self.current_token {
    //         Some(Token::Number(n)) => match u32::try_from(*n) {
    //             Ok(n) => Some(n),
    //             Err(_) => {
    //                 return Err(Error {
    //                     kind: ErrorKind::ExpectedUnsigned,
    //                     line: self.lexer.current_line(),
    //                 });
    //             }
    //         },
    //         _ => None,
    //     };

    //     if line_number.is_some() {
    //         self.current_token = self.lexer.next();
    //     }

    //     Ok(Statement::Restore { line_number })
    // }

    // fn poke(&mut self) -> Result<Statement, Error> {
    //     self.current_token = self.lexer.next();
    //     let address = match &self.current_token {
    //         Some(Token::Number(n)) => u32::try_from(*n).map_err(|_e| Error {
    //             kind: ErrorKind::ExpectedUnsigned,
    //             line: self.lexer.current_line(),
    //         })?,
    //         _ => {
    //             return Err(Error {
    //                 kind: ErrorKind::ExpectedUnsigned,
    //                 line: self.lexer.current_line(),
    //             });
    //         }
    //     };

    //     self.current_token = self.lexer.next();
    //     if self.current_token != Some(Token::Comma) {
    //         return Err(Error {
    //             kind: ErrorKind::UnexpectedToken,
    //             line: self.lexer.current_line(),
    //         });
    //     }

    //     self.current_token = self.lexer.next();
    //     let mut values: Vec<u8> = Vec::new();

    //     loop {
    //         match &mut self.current_token {
    //             Some(Token::Number(n)) => {
    //                 values.push(u8::try_from(*n).map_err(|_e| Error {
    //                     kind: ErrorKind::ExpectedUnsigned,
    //                     line: self.lexer.current_line(),
    //                 })?);
    //                 self.current_token = self.lexer.next();
    //             }
    //             _ => {
    //                 return Err(Error {
    //                     kind: ErrorKind::ExpectedUnsigned,
    //                     line: self.lexer.current_line(),
    //                 });
    //             }
    //         }

    //         if self.current_token == Some(Token::Comma) {
    //             self.current_token = self.lexer.next();
    //         } else {
    //             break;
    //         }
    //     }

    //     Ok(Statement::Poke { address, values })
    // }

    // fn call(&mut self) -> Result<Statement, Error> {
    //     self.current_token = self.lexer.next();
    //     let address = match &self.current_token {
    //         Some(Token::Number(n)) => u32::try_from(*n).map_err(|_e| Error {
    //             kind: ErrorKind::ExpectedUnsigned,
    //             line: self.lexer.current_line(),
    //         })?,
    //         _ => {
    //             return Err(Error {
    //                 kind: ErrorKind::ExpectedUnsigned,
    //                 line: self.lexer.current_line(),
    //             });
    //         }
    //     };

    //     self.current_token = self.lexer.next();

    //     Ok(Statement::Call { address })
    // }

    // fn goto(&mut self) -> Result<Statement, Error> {
    //     self.current_token = self.lexer.next();
    //     let line_number = match &self.current_token {
    //         Some(Token::Number(n)) => match u32::try_from(*n) {
    //             Ok(n) => n,
    //             Err(_) => {
    //                 return Err(Error {
    //                     kind: ErrorKind::ExpectedUnsigned,
    //                     line: self.lexer.current_line(),
    //                 });
    //             }
    //         },
    //         _ => {
    //             return Err(Error {
    //                 kind: ErrorKind::ExpectedUnsigned,
    //                 line: self.lexer.current_line(),
    //             });
    //         }
    //     };

    //     self.current_token = self.lexer.next();

    //     Ok(Statement::Goto { line_number })
    // }

    // fn gosub(&mut self) -> Result<Statement, Error> {
    //     self.current_token = self.lexer.next();
    //     let line_number = match &self.current_token {
    //         Some(Token::Number(n)) => match u32::try_from(*n) {
    //             Ok(n) => n,
    //             Err(_) => {
    //                 return Err(Error {
    //                     kind: ErrorKind::ExpectedUnsigned,
    //                     line: self.lexer.current_line(),
    //                 });
    //             }
    //         },
    //         _ => {
    //             return Err(Error {
    //                 kind: ErrorKind::ExpectedUnsigned,
    //                 line: self.lexer.current_line(),
    //             });
    //         }
    //     };

    //     self.current_token = self.lexer.next();

    //     Ok(Statement::GoSub { line_number })
    // }

    // fn return_(&mut self) -> Result<Statement, Error> {
    //     self.current_token = self.lexer.next();

    //     Ok(Statement::Return)
    // }

    // fn if_(&mut self) -> Result<Statement, Error> {
    //     self.current_token = self.lexer.next();
    //     let condition = match self.expression()? {
    //         Some(expr) => expr,
    //         None => {
    //             return Err(Error {
    //                 kind: ErrorKind::ExpectedExpression,
    //                 line: self.lexer.current_line(),
    //             });
    //         }
    //     };

    //     if self.current_token == Some(Token::Then) {
    //         self.current_token = self.lexer.next();
    //     }

    //     let then = Box::new(self.statement()?);

    //     let else_ = if self.current_token == Some(Token::Else) {
    //         self.current_token = self.lexer.next();
    //         let statement = self.statement()?;
    //         Some(Box::new(statement))
    //     } else {
    //         None
    //     };

    //     Ok(Statement::If {
    //         condition,
    //         then,
    //         else_,
    //     })
    // }

    // fn for_(&mut self) -> Result<Statement, Error> {
    //     self.current_token = self.lexer.next();
    //     let variable = match &mut self.current_token {
    //         Some(Token::Identifier(v)) => mem::take(v),
    //         _ => {
    //             return Err(Error {
    //                 kind: ErrorKind::ExpectedIdentifier,
    //                 line: self.lexer.current_line(),
    //             });
    //         }
    //     };

    //     self.current_token = self.lexer.next();
    //     if self.current_token != Some(Token::Equal) {
    //         return Err(Error {
    //             kind: ErrorKind::UnexpectedToken,
    //             line: self.lexer.current_line(),
    //         });
    //     }

    //     self.current_token = self.lexer.next();
    //     let from = match self.expression()? {
    //         Some(expr) => expr,
    //         None => {
    //             return Err(Error {
    //                 kind: ErrorKind::ExpectedExpression,
    //                 line: self.lexer.current_line(),
    //             });
    //         }
    //     };

    //     if self.current_token != Some(Token::To) {
    //         return Err(Error {
    //             kind: ErrorKind::UnexpectedToken,
    //             line: self.lexer.current_line(),
    //         });
    //     }

    //     self.current_token = self.lexer.next();
    //     let to = match self.expression()? {
    //         Some(expr) => expr,
    //         None => {
    //             return Err(Error {
    //                 kind: ErrorKind::ExpectedExpression,
    //                 line: self.lexer.current_line(),
    //             });
    //         }
    //     };

    //     let step = if self.current_token == Some(Token::Step) {
    //         self.current_token = self.lexer.next();
    //         match self.expression()? {
    //             Some(expr) => Some(expr),
    //             None => {
    //                 return Err(Error {
    //                     kind: ErrorKind::ExpectedExpression,
    //                     line: self.lexer.current_line(),
    //                 });
    //             }
    //         }
    //     } else {
    //         None
    //     };

    //     Ok(Statement::For {
    //         variable,
    //         from,
    //         to,
    //         step,
    //     })
    // }

    // fn next(&mut self) -> Result<Statement, Error> {
    //     self.current_token = self.lexer.next();
    //     let variable = match &mut self.current_token {
    //         Some(Token::Identifier(v)) => mem::take(v),
    //         _ => {
    //             return Err(Error {
    //                 kind: ErrorKind::ExpectedIdentifier,
    //                 line: self.lexer.current_line(),
    //             });
    //         }
    //     };

    //     self.current_token = self.lexer.next();

    //     Ok(Statement::Next { variable })
    // }

    // fn end(&mut self) -> Result<Statement, Error> {
    //     self.current_token = self.lexer.next();

    //     Ok(Statement::End)
    // }

    // fn comment(&mut self) -> Result<Statement, Error> {
    //     match &mut self.current_token {
    //         Some(Token::Rem(s)) => {
    //             let res = Ok(Statement::Rem {
    //                 content: mem::take(s),
    //             });

    //             self.current_token = self.lexer.next();

    //             res
    //         }
    //         _ => {
    //             unreachable!("We already checked for REM");
    //         }
    //     }
    // }

    // fn dim(&mut self) -> Result<Statement, Error> {
    //     self.current_token = self.lexer.next();
    //     let variable = match &mut self.current_token {
    //         Some(Token::Identifier(v)) => mem::take(v),
    //         _ => {
    //             return Err(Error {
    //                 kind: ErrorKind::ExpectedIdentifier,
    //                 line: self.lexer.current_line(),
    //             });
    //         }
    //     };

    //     self.current_token = self.lexer.next();
    //     if self.current_token != Some(Token::LeftParen) {
    //         return Err(Error {
    //             kind: ErrorKind::ExpectedLeftParen,
    //             line: self.lexer.current_line(),
    //         });
    //     }

    //     self.current_token = self.lexer.next();
    //     let size = match &self.current_token {
    //         Some(Token::Number(n)) => match u32::try_from(*n) {
    //             Ok(n) => n,
    //             Err(_) => {
    //                 return Err(Error {
    //                     kind: ErrorKind::ExpectedUnsigned,
    //                     line: self.lexer.current_line(),
    //                 });
    //             }
    //         },
    //         _ => {
    //             return Err(Error {
    //                 kind: ErrorKind::ExpectedUnsigned,
    //                 line: self.lexer.current_line(),
    //             });
    //         }
    //     };

    //     if self.current_token != Some(Token::RightParen) {
    //         return Err(Error {
    //             kind: ErrorKind::ExpectedRightParen,
    //             line: self.lexer.current_line(),
    //         });
    //     }

    //     self.current_token = self.lexer.next();

    //     let length = if self.current_token == Some(Token::Star) {
    //         self.current_token = self.lexer.next();
    //         match &self.current_token {
    //             Some(Token::Number(n)) => match u32::try_from(*n) {
    //                 Ok(n) => {
    //                     self.current_token = self.lexer.next();
    //                     Some(n)
    //                 }
    //                 Err(_) => {
    //                     return Err(Error {
    //                         kind: ErrorKind::ExpectedUnsigned,
    //                         line: self.lexer.current_line(),
    //                     });
    //                 }
    //             },
    //             _ => {
    //                 return Err(Error {
    //                     kind: ErrorKind::ExpectedUnsigned,
    //                     line: self.lexer.current_line(),
    //                 });
    //             }
    //         }
    //     } else {
    //         None
    //     };

    //     Ok(Statement::Dim {
    //         variable,
    //         size,
    //         length,
    //     })
    // }

    // fn atomic_statement(&mut self) -> Result<Statement, Error> {
    //     // println!("Atomic statement: {:?}", self.current_token);
    //     match self.current_token {
    //         Some(Token::Let | Token::Identifier(_)) => self.let_(),
    //         Some(Token::Print) => self.print(),
    //         Some(Token::Pause) => self.pause(),
    //         Some(Token::Input) => self.input(),
    //         Some(Token::Wait) => self.wait(),
    //         Some(Token::Goto) => self.goto(),
    //         Some(Token::For) => self.for_(),
    //         Some(Token::Next) => self.next(),
    //         Some(Token::End) => self.end(),
    //         Some(Token::Gosub) => self.gosub(),
    //         Some(Token::If) => self.if_(),
    //         Some(Token::Return) => self.return_(),
    //         Some(Token::Data) => self.data(),
    //         Some(Token::Read) => self.read(),
    //         Some(Token::Restore) => self.restore(),
    //         Some(Token::Poke) => self.poke(),
    //         Some(Token::Call) => self.call(),
    //         Some(Token::Dim) => self.dim(),
    //         Some(Token::Rem(_)) => self.comment(),
    //         _ => Err(Error {
    //             kind: ErrorKind::ExpectedStatement,
    //             line: self.lexer.current_line(),
    //         }),
    //     }
    // }

    // fn statement(&mut self) -> Result<Statement, Error> {
    //     // TODO: small vec optimization
    //     let mut statements = Vec::new();

    //     loop {
    //         let stmt = self.atomic_statement()?;

    //         statements.push(stmt);

    //         if self.current_token == Some(Token::Colon) {
    //             self.current_token = self.lexer.next();
    //         } else {
    //             break;
    //         }
    //     }

    //     Ok(if statements.len() == 1 {
    //         statements.remove(0)
    //     } else {
    //         Statement::Seq { statements }
    //     })
    // }

    // fn line(&mut self) -> Result<(u32, Statement), Error> {
    //     let line_number = match &self.current_token {
    //         Some(Token::Number(n)) => {
    //             if let Ok(n) = u32::try_from(*n) {
    //                 n
    //             } else {
    //                 return Err(Error {
    //                     kind: ErrorKind::ExpectedLineNumber,
    //                     line: self.lexer.current_line(),
    //                 });
    //             }
    //         }
    //         _ => {
    //             return Err(Error {
    //                 kind: ErrorKind::ExpectedLineNumber,
    //                 line: self.lexer.current_line(),
    //             })
    //         }
    //     };

    //     self.current_token = self.lexer.next();
    //     let statement = self.statement()?;

    //     match self.current_token {
    //         Some(Token::Newline) => {
    //             self.current_token = self.lexer.next();
    //         }
    //         None => {}
    //         _ => {
    //             return Err(Error {
    //                 kind: ErrorKind::ExpectedEndOfLine,
    //                 line: self.lexer.current_line(),
    //             });
    //         }
    //     }

    //     Ok((line_number, statement))
    // }

    // fn program(&mut self) -> (Program, Vec<Error>) {
    //     let mut errors = Vec::new();
    //     let mut program = Program::new();

    //     self.current_token = self.lexer.next();

    //     while self.current_token.is_some() {
    //         match self.line() {
    //             Ok((line_number, statement)) => {
    //                 program.add_line(line_number, statement);
    //             }
    //             Err(e) => {
    //                 errors.push(e);
    //                 self.current_token = self.lexer.next();

    //                 while self.current_token != Some(Token::Newline) {
    //                     self.current_token = self.lexer.next();
    //                 }
    //             }
    //         }
    //     }

    //     (program, errors)
    // }
}

mod expression {
    use crate::ast::{
        error::ErrorKind, node::LValue, BinaryOperator, Error, Expression, UnaryOperator,
    };
    use crate::tokens::{Lexer, Token};
    use std::{iter::Peekable, mem};

    pub struct ExpressionParser<'a> {
        lexer: Peekable<Lexer<'a>>,
    }

    impl<'a> ExpressionParser<'a> {
        pub fn new(lexer: Peekable<Lexer<'a>>) -> Self {
            Self { lexer }
        }

        pub fn parse(&mut self) -> Result<Option<Expression>, Error> {
            // println!("expression");
            self.comparison()
        }

        fn lvalue(&mut self) -> Result<LValue, Error> {
            // println!("lvalue");
            match self.lexer.peek_mut() {
                Some(Token::Identifier(v)) => {
                    let variable = mem::take(v);
                    let next = self.lexer.next();

                    // println!("identifier {}", v);

                    if next == Some(Token::LeftParen) {
                        self.lexer.next();
                        let index = self.parse()?;
                        if self.lexer.peek() == Some(&Token::RightParen) {
                            let res = Ok(LValue::ArrayElement {
                                variable,
                                index: Box::new(index.unwrap()),
                            });

                            self.lexer.next();

                            res
                        } else {
                            Err(Error {
                                kind: ErrorKind::MismatchedParentheses,
                                line: 0, // TODO
                            })
                        }
                    } else {
                        Ok(LValue::Variable(variable))
                    }
                }
                _ => {
                    // println!("expected identifier");
                    Err(Error {
                        kind: ErrorKind::ExpectedIdentifier,
                        line: 0, // TODO
                    })
                }
            }
        }

        fn term(&mut self) -> Result<Option<Expression>, Error> {
            match self.lexer.peek_mut() {
                Some(Token::Number(n)) => {
                    let res = Ok(Some(Expression::Number(*n)));
                    self.lexer.next();
                    res
                }
                Some(Token::Identifier(_)) => self.lvalue().map(|v| Some(Expression::LValue(v))),
                Some(Token::String(s)) => {
                    let res = Ok(Some(Expression::String(mem::take(s))));
                    self.lexer.next();
                    res
                }
                Some(Token::LeftParen) => {
                    self.lexer.next();
                    let res = self.parse()?;
                    // if self.lexer.next() == Some(Token::RightParen) {
                    //     Ok(res)
                    // } else {
                    //     Err(Error {
                    //         kind: ErrorKind::MismatchedParentheses,
                    //         line: 0, // TODO
                    //     })
                    // }
                    Ok(res)
                }
                _ => Ok(None),
            }
        }

        // unary + and -
        fn factor(&mut self) -> Result<Option<Expression>, Error> {
            // println!("factor");
            if self.lexer.peek() == Some(&Token::Plus) || self.lexer.peek() == Some(&Token::Minus) {
                let op = match self.lexer.next() {
                    Some(Token::Plus) => UnaryOperator::Plus,
                    Some(Token::Minus) => UnaryOperator::Minus,
                    _ => unreachable!(),
                };

                let operand = self.factor();
                let operand = if let Some(operand) = operand? {
                    operand
                } else {
                    return Err(Error {
                        kind: ErrorKind::ExpectedExpression,
                        line: 0, // TODO
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

            while let Some(&Token::Star) | Some(&Token::Slash) = self.lexer.peek() {
                let op = match self.lexer.next() {
                    Some(Token::Star) => BinaryOperator::Mul,
                    Some(Token::Slash) => BinaryOperator::Div,
                    _ => unreachable!(),
                };

                let right = self.factor();
                let right = if let Some(right) = right? {
                    right
                } else {
                    return Err(Error {
                        kind: ErrorKind::ExpectedExpression,
                        line: 0, // TODO
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

            while let Some(&Token::Plus) | Some(&Token::Minus) = self.lexer.peek() {
                println!("add_sub");

                let op = match self.lexer.next() {
                    Some(Token::Plus) => BinaryOperator::Add,
                    Some(Token::Minus) => BinaryOperator::Sub,
                    _ => unreachable!(),
                };

                let right = self.mul_div();
                let right = if let Some(right) = right? {
                    right
                } else {
                    return Err(Error {
                        kind: ErrorKind::ExpectedExpression,
                        line: 0, // TODO
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

            while let Some(&Token::Equal)
            | Some(&Token::Diamond)
            | Some(&Token::LessThan)
            | Some(&Token::LessOrEqual)
            | Some(&Token::GreaterThan)
            | Some(&Token::GreaterOrEqual) = self.lexer.peek()
            {
                let op = match self.lexer.next() {
                    Some(Token::Equal) => BinaryOperator::Eq,
                    Some(Token::Diamond) => BinaryOperator::Ne,
                    Some(Token::LessThan) => BinaryOperator::Lt,
                    Some(Token::LessOrEqual) => BinaryOperator::Le,
                    Some(Token::GreaterThan) => BinaryOperator::Gt,
                    Some(Token::GreaterOrEqual) => BinaryOperator::Ge,
                    _ => unreachable!(),
                };

                let right = self.add_sub();
                let right = if let Some(right) = right? {
                    right
                } else {
                    return Err(Error {
                        kind: ErrorKind::ExpectedExpression,
                        line: 0, // TODO
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
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn add_sub_1() {
            let expected = Expression::Binary {
                left: Box::new(Expression::Binary {
                    left: Box::new(Expression::Number(1)),
                    op: BinaryOperator::Add,
                    right: Box::new(Expression::Number(2)),
                }),
                op: BinaryOperator::Sub,
                right: Box::new(Expression::Number(3)),
            };

            let lexer = Lexer::new("1 + 2 - 3");
            let mut parser = ExpressionParser::new(lexer.peekable());

            let res = parser
                .add_sub()
                .expect("Failed to parse expression")
                .expect("Expected an expression");

            assert_eq!(res, expected);
        }

        // FIXME: Check operator precedence
        #[test]
        fn mul_div_1() {
            let expected = Expression::Binary {
                left: Box::new(Expression::Binary {
                    left: Box::new(Expression::Number(1)),
                    op: BinaryOperator::Mul,
                    right: Box::new(Expression::Number(2)),
                }),
                op: BinaryOperator::Div,
                right: Box::new(Expression::Number(3)),
            };

            let lexer = Lexer::new("1 * 2 / 3");
            let mut parser = ExpressionParser::new(lexer.peekable());

            let res = parser
                .mul_div()
                .expect("Failed to parse expression")
                .expect("Expected an expression");

            assert_eq!(res, expected);
        }

        #[test]
        fn lvalue_1() {
            let expected = LValue::Variable("A".to_owned());

            let lexer = Lexer::new("A");
            let mut parser = ExpressionParser::new(lexer.peekable());

            let res = parser.lvalue().expect("Failed to parse lvalue");

            assert_eq!(res, expected);
        }

        #[test]
        fn factor_1() {
            let expected = Expression::Number(42);

            let lexer = Lexer::new("42");
            let mut parser = ExpressionParser::new(lexer.peekable());

            let res = parser
                .factor()
                .expect("Failed to parse expression")
                .expect("Expected an expression");

            assert_eq!(res, expected);
        }

        // Unary +
        #[test]
        fn factor_2() {
            let expected = Expression::Unary {
                op: UnaryOperator::Plus,
                operand: Box::new(Expression::Number(42)),
            };

            let lexer = Lexer::new("+42");
            let mut parser = ExpressionParser::new(lexer.peekable());

            let res = parser
                .factor()
                .expect("Failed to parse expression")
                .expect("Expected an expression");

            assert_eq!(res, expected);
        }

        // Unary -
        #[test]
        fn factor_3() {
            let expected = Expression::Unary {
                op: UnaryOperator::Minus,
                operand: Box::new(Expression::Number(42)),
            };

            let lexer = Lexer::new("-42");
            let mut parser = ExpressionParser::new(lexer.peekable());

            let res = parser
                .factor()
                .expect("Failed to parse expression")
                .expect("Expected an expression");

            assert_eq!(res, expected);
        }
    }

    // Parenthesized expression
    #[test]
    fn term_1() {
        let expected = Expression::Binary {
            left: Box::new(Expression::Number(42)),
            op: BinaryOperator::Mul,
            right: Box::new(Expression::Number(43)),
        };

        let lexer = Lexer::new("(42 * 43)");

        let mut parser = ExpressionParser::new(lexer.peekable());

        let res = parser
            .term()
            .expect("Failed to parse expression")
            .expect("Expected an expression");

        assert_eq!(res, expected);
    }

    #[test]
    fn comparison_eq() {
        let expected = Expression::Binary {
            left: Box::new(Expression::Number(42)),
            op: BinaryOperator::Eq,
            right: Box::new(Expression::Number(43)),
        };

        let lexer = Lexer::new("42 = 43");
        let mut parser = ExpressionParser::new(lexer.peekable());

        let res = parser
            .comparison()
            .expect("Failed to parse expression")
            .expect("Expected an expression");

        assert_eq!(res, expected);
    }
}
