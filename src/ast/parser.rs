use crate::tokens::{Lexer, Token};

use super::{BinaryOperator, Expression, Program, Statement};

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

    pub fn parse(&mut self) -> Program {
        self.current_token = self.lexer.next_token();
        self.parse_program()
    }

    fn parse_factor(&mut self) -> Option<Expression> {
        match &self.current_token {
            Some(Token::Number(n)) => {
                let n = *n;
                self.current_token = self.lexer.next_token();
                Some(Expression::NumberLiteral(n))
            }
            Some(Token::Identifier(v)) => {
                let v = v.clone();
                self.current_token = self.lexer.next_token();
                Some(Expression::Variable(v))
            }
            Some(Token::String(s)) => {
                let s = s.clone();
                self.current_token = self.lexer.next_token();
                Some(Expression::StringLiteral(s))
            }
            Some(Token::LParen) => {
                self.current_token = self.lexer.next_token();
                let res = self.parse_expression();
                if self.current_token == Some(Token::RParen) {
                    self.current_token = self.lexer.next_token();
                } else {
                    panic!("Expected closing parenthesis");
                }
                res
            }
            _ => None,
        }
    }

    fn parse_mul_div(&mut self) -> Option<Expression> {
        let mut left = self.parse_factor()?;

        while let Some(Token::Star) | Some(Token::Slash) = self.current_token {
            let op = match self.current_token {
                Some(Token::Star) => BinaryOperator::Mul,
                Some(Token::Slash) => BinaryOperator::Div,
                _ => unreachable!(),
            };

            self.current_token = self.lexer.next_token();
            let right = self.parse_factor();
            let right = if let Some(right) = right {
                right
            } else {
                panic!("Expected expression after operator {}", op);
            };

            left = Expression::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Some(left)
    }

    fn parse_add_sub(&mut self) -> Option<Expression> {
        let mut left = self.parse_mul_div()?;

        while let Some(Token::Plus) | Some(Token::Minus) = self.current_token {
            let op = match self.current_token {
                Some(Token::Plus) => BinaryOperator::Add,
                Some(Token::Minus) => BinaryOperator::Sub,
                _ => unreachable!(),
            };

            self.current_token = self.lexer.next_token();
            let right = self.parse_mul_div();
            let right = if let Some(right) = right {
                right
            } else {
                panic!("Expected expression after operator {}", op);
            };

            left = Expression::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Some(left)
    }

    fn parse_comparison(&mut self) -> Option<Expression> {
        let mut left = self.parse_add_sub()?;

        while let Some(Token::Eq) | Some(Token::Diamond) | Some(Token::Lt) | Some(Token::Le)
        | Some(Token::Gt) | Some(Token::Ge) = self.current_token
        {
            let op = match self.current_token {
                Some(Token::Eq) => BinaryOperator::Eq,
                Some(Token::Diamond) => BinaryOperator::Ne,
                Some(Token::Lt) => BinaryOperator::Lt,
                Some(Token::Le) => BinaryOperator::Le,
                Some(Token::Gt) => BinaryOperator::Gt,
                Some(Token::Ge) => BinaryOperator::Ge,
                _ => unreachable!(),
            };

            self.current_token = self.lexer.next_token();
            let right = self.parse_add_sub();
            let right = if let Some(right) = right {
                right
            } else {
                panic!("Expected expression after operator {}", op);
            };

            left = Expression::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Some(left)
    }

    fn parse_and(&mut self) -> Option<Expression> {
        let mut left = self.parse_comparison()?;

        while self.current_token == Some(Token::And) {
            self.current_token = self.lexer.next_token();
            let right = self.parse_comparison();
            let right = if let Some(right) = right {
                right
            } else {
                panic!("Expected expression after AND");
            };

            left = Expression::Binary {
                left: Box::new(left),
                op: BinaryOperator::And,
                right: Box::new(right),
            };
        }

        Some(left)
    }

    fn parse_or(&mut self) -> Option<Expression> {
        let mut left = self.parse_and()?;

        while self.current_token == Some(Token::Or) {
            self.current_token = self.lexer.next_token();
            let right = self.parse_and();
            let right = if let Some(right) = right {
                right
            } else {
                panic!("Expected expression after OR");
            };

            left = Expression::Binary {
                left: Box::new(left),
                op: BinaryOperator::Or,
                right: Box::new(right),
            };
        }

        Some(left)
    }

    fn parse_expression(&mut self) -> Option<Expression> {
        self.parse_or()
    }

    fn parse_let(&mut self) -> Option<Statement> {
        let variable = match &self.current_token {
            Some(Token::Let) => {
                self.current_token = self.lexer.next_token();

                let variable = match &self.current_token {
                    Some(Token::Identifier(v)) => v.clone(),
                    _ => panic!("Expected variable name after LET"),
                };

                variable
            }
            Some(Token::Identifier(v)) => v.clone(),
            _ => {
                // LET keyword is optional
                return None;
            }
        };

        self.current_token = self.lexer.next_token();
        if self.current_token != Some(Token::Eq) {
            panic!("Expected = after variable name");
        }

        self.current_token = self.lexer.next_token();
        let expression = self.parse_expression();
        let expression = if let Some(expression) = expression {
            expression
        } else {
            panic!("Expected expression after =");
        };

        Some(Statement::Let {
            variable,
            expression,
        })
    }

    fn parse_print(&mut self) -> Option<Statement> {
        if self.current_token != Some(Token::Print) {
            return None;
        }

        self.current_token = self.lexer.next_token();
        let mut content = Vec::new();

        while let Some(expr) = self.parse_expression() {
            content.push(expr);

            if self.current_token == Some(Token::Semicolon) {
                self.current_token = self.lexer.next_token();
            } else {
                break;
            }
        }

        Some(Statement::Print { content })
    }

    fn parse_input(&mut self) -> Option<Statement> {
        if self.current_token != Some(Token::Input) {
            return None;
        }

        self.current_token = self.lexer.next_token();
        let prompt = self.parse_expression();

        if self.current_token == Some(Token::Semicolon) {
            self.current_token = self.lexer.next_token();
        }

        let variable = match &self.current_token {
            Some(Token::Identifier(v)) => v.clone(),
            _ => panic!("Expected variable name after INPUT"),
        };

        self.current_token = self.lexer.next_token();

        Some(Statement::Input { prompt, variable })
    }

    fn parse_goto(&mut self) -> Option<Statement> {
        if self.current_token != Some(Token::Goto) {
            return None;
        }

        self.current_token = self.lexer.next_token();
        let line_number = match &self.current_token {
            Some(Token::Number(n)) => match u32::try_from(*n) {
                Ok(n) => n,
                Err(e) => {
                    panic!("Goto line label must be convertible to u32: {:?}", e);
                }
            },
            _ => panic!("Expected line number after GOTO"),
        };

        self.current_token = self.lexer.next_token();

        Some(Statement::Goto { line_number })
    }

    fn parse_gosub(&mut self) -> Option<Statement> {
        if self.current_token != Some(Token::Gosub) {
            return None;
        }

        self.current_token = self.lexer.next_token();
        let line_number = match &self.current_token {
            Some(Token::Number(n)) => match u32::try_from(*n) {
                Ok(n) => n,
                Err(e) => {
                    panic!("Gosub line label must be convertible to u32: {:?}", e);
                }
            },
            _ => panic!("Expected line number after GOSUB"),
        };

        self.current_token = self.lexer.next_token();

        Some(Statement::GoSub { line_number })
    }

    fn parse_return(&mut self) -> Option<Statement> {
        if self.current_token != Some(Token::Return) {
            return None;
        }

        self.current_token = self.lexer.next_token();

        Some(Statement::Return)
    }

    fn parse_if(&mut self) -> Option<Statement> {
        if self.current_token != Some(Token::If) {
            return None;
        }

        self.current_token = self.lexer.next_token();
        let condition = match self.parse_expression() {
            Some(expr) => expr,
            None => panic!("Expected expression after IF"),
        };

        if self.current_token == Some(Token::Then) {
            self.current_token = self.lexer.next_token();
        }

        let then = match self.parse_statement() {
            Some(stmt) => Box::new(stmt),
            None => panic!("Expected statement after THEN"),
        };

        let else_ = if self.current_token == Some(Token::Else) {
            self.current_token = self.lexer.next_token();
            match self.parse_statement() {
                Some(stmt) => Some(Box::new(stmt)),
                None => panic!("Expected statement after ELSE"),
            }
        } else {
            None
        };

        Some(Statement::If {
            condition,
            then,
            else_,
        })
    }

    fn parse_for(&mut self) -> Option<Statement> {
        if self.current_token != Some(Token::For) {
            return None;
        }

        self.current_token = self.lexer.next_token();
        let variable = match &self.current_token {
            Some(Token::Identifier(v)) => v.clone(),
            _ => panic!("Expected variable name after FOR"),
        };

        self.current_token = self.lexer.next_token();
        if self.current_token != Some(Token::Eq) {
            panic!("Expected = after variable name");
        }

        self.current_token = self.lexer.next_token();
        let from = match self.parse_expression() {
            Some(expr) => expr,
            None => panic!("Expected expression after ="),
        };

        if self.current_token != Some(Token::To) {
            panic!("Expected TO after FROM");
        }

        self.current_token = self.lexer.next_token();
        let to = match self.parse_expression() {
            Some(expr) => expr,
            None => panic!("Expected expression after TO"),
        };

        let step = if self.current_token == Some(Token::Step) {
            self.current_token = self.lexer.next_token();
            match self.parse_expression() {
                Some(expr) => Some(expr),
                None => panic!("Expected expression after STEP"),
            }
        } else {
            None
        };

        Some(Statement::For {
            variable,
            from,
            to,
            step,
        })
    }

    fn parse_next(&mut self) -> Option<Statement> {
        if self.current_token != Some(Token::Next) {
            return None;
        }

        self.current_token = self.lexer.next_token();
        let variable = match &self.current_token {
            Some(Token::Identifier(v)) => v.clone(),
            _ => panic!("Expected variable name after NEXT"),
        };

        self.current_token = self.lexer.next_token();

        Some(Statement::Next { variable })
    }

    fn parse_end(&mut self) -> Option<Statement> {
        if self.current_token != Some(Token::End) {
            return None;
        }

        self.current_token = self.lexer.next_token();

        Some(Statement::End)
    }

    fn parse_atomic_statement(&mut self) -> Option<Statement> {
        self.parse_let()
            .or_else(|| self.parse_print())
            .or_else(|| self.parse_input())
            .or_else(|| self.parse_goto())
            .or_else(|| self.parse_for())
            .or_else(|| self.parse_next())
            .or_else(|| self.parse_end())
            .or_else(|| self.parse_gosub())
            .or_else(|| self.parse_if())
            .or_else(|| self.parse_return())
            .or_else(|| self.parse_comment())
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        let mut statements = Vec::new();

        while let Some(stmt) = self.parse_atomic_statement() {
            statements.push(stmt);

            if self.current_token == Some(Token::Colon) {
                self.current_token = self.lexer.next_token();
            } else {
                break;
            }
        }

        if statements.len() == 1 {
            Some(statements.remove(0))
        } else {
            Some(Statement::Seq { statements })
        }
    }

    fn parse_comment(&mut self) -> Option<Statement> {
        match &self.current_token {
            Some(Token::Rem(s)) => {
                let s = s.clone();
                self.current_token = self.lexer.next_token();
                Some(Statement::Rem { content: s })
            }
            _ => None,
        }
    }

    fn parse_line(&mut self) -> Option<(u32, Statement)> {
        let line_number = match &self.current_token {
            Some(Token::Number(n)) => {
                if let Ok(n) = u32::try_from(*n) {
                    n
                } else {
                    panic!("Line number must be convertible to u32");
                }
            }
            _ => return None,
        };

        self.current_token = self.lexer.next_token();
        let statement = match self.parse_statement() {
            Some(stmt) => stmt,
            None => panic!("Expected statement after line number"),
        };

        match self.current_token {
            Some(Token::Eol) => {
                self.current_token = self.lexer.next_token();
            }
            None => {}
            _ => panic!("Expected end of line, found {:?}", self.current_token),
        }

        Some((line_number, statement))
    }

    fn parse_program(&mut self) -> Program {
        let mut program = Program::new();

        while let Some((line_number, statement)) = self.parse_line() {
            program.add_line(line_number, statement);
        }

        program
    }
}
