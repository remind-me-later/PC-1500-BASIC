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
