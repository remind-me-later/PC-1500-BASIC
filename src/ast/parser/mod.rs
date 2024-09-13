mod expression;

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
