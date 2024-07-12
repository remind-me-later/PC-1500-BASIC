use super::lexer::{Lexer, Tok};

#[derive(Debug, Clone)]
pub struct Program {
    lines: Vec<Line>,
}

impl IntoIterator for Program {
    type Item = Line;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.lines.into_iter()
    }
}

impl std::fmt::Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.lines {
            writeln!(f, "{}", line)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Line {
    line_number: u64,
    stmt: Stmt,
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.line_number, self.stmt)
    }
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Let {
        var: String,
        ex: Box<Expr>,
    },
    For {
        var: String,
        start: Box<Expr>,
        cond: Box<Expr>,
        step: Box<Expr>,
    },
    Next {
        var: String,
    },
    Print {
        ex: Box<Expr>,
    },
    End,
    Rem {
        comment: String,
    },
}

impl std::fmt::Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::Let { var, ex } => write!(f, "LET {} = {}", var, ex),
            Stmt::For {
                var,
                start,
                cond,
                step,
            } => {
                write!(f, "FOR {} = {} TO {} STEP {}", var, start, cond, step)
            }
            Stmt::Next { var } => write!(f, "NEXT {}", var),
            Stmt::Print { ex } => write!(f, "PRINT {}", ex),
            Stmt::End => write!(f, "END"),
            Stmt::Rem { comment } => write!(f, "REM {}", comment),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    BinOp {
        lhs: Box<Expr>,
        op: Tok,
        rhs: Box<Expr>,
    },
    Num(u64),
    StringLiteral(String),
    Var(String),
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::BinOp { lhs, op, rhs } => {
                let op_str = match op {
                    Tok::Plus => "+",
                    Tok::Minus => "-",
                    Tok::Star => "*",
                    Tok::Slash => "/",
                    _ => panic!("Unexpected token in BinOp"),
                };
                write!(f, "({} {} {})", lhs, op_str, rhs)
            }
            Expr::Num(value) => write!(f, "{}", value),
            Expr::Var(name) => write!(f, "{}", name),
            Expr::StringLiteral(value) => write!(f, "\"{}\"", value),
        }
    }
}

pub struct Parser {
    lexer: Lexer,
    current_token: Tok,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next_token();
        Parser {
            lexer,
            current_token,
        }
    }

    pub fn parse(&mut self) -> Program {
        self.prgrm()
    }

    // Helper functions
    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    fn eat(&mut self, token: Tok) {
        if self.current_token == token {
            self.advance();
        } else {
            panic!(
                "Expected token: {:?}, found: {:?}",
                token, self.current_token
            );
        }
    }

    // Expression parsing functions
    fn factor(&mut self) -> Expr {
        match self.current_token.clone() {
            Tok::Number(value) => {
                self.advance();
                Expr::Num(value)
            }
            Tok::StringLiteral(value) => {
                self.advance();
                Expr::StringLiteral(value)
            }
            Tok::Identifier(name) => {
                self.advance();
                Expr::Var(name)
            }
            Tok::LParen => {
                self.advance();
                let node = self.expr();
                self.eat(Tok::RParen);
                node
            }
            _ => panic!(
                "Expected number, identifier, or LParen, found: {:?}",
                self.current_token
            ),
        }
    }

    fn term(&mut self) -> Expr {
        let mut node = self.factor();
        while self.current_token == Tok::Star || self.current_token == Tok::Slash {
            let token = self.current_token.clone();
            self.advance();
            node = Expr::BinOp {
                lhs: Box::new(node),
                op: token,
                rhs: Box::new(self.factor()),
            };
        }
        node
    }

    fn expr(&mut self) -> Expr {
        let mut node = self.term();
        while self.current_token == Tok::Plus || self.current_token == Tok::Minus {
            let token = self.current_token.clone();
            self.advance();
            node = Expr::BinOp {
                lhs: Box::new(node),
                op: token,
                rhs: Box::new(self.term()),
            };
        }
        node
    }

    // Line parsing functions
    fn line(&mut self) -> Line {
        let line_number = match self.current_token.clone() {
            Tok::Number(value) => {
                self.advance();
                value
            }
            _ => panic!("Expected line number, found: {:?}", self.current_token),
        };

        let stmt = self.stmt();

        Line { line_number, stmt }
    }

    // Statement parsing functions
    fn let_stmt(&mut self) -> Stmt {
        if let Tok::Identifier(var) = self.current_token.clone() {
            self.advance();
            self.eat(Tok::Assign);
            Stmt::Let {
                var,
                ex: Box::new(self.expr()),
            }
        } else {
            panic!("Expected identifier, found: {:?}", self.current_token);
        }
    }

    fn for_stmt(&mut self) -> Stmt {
        if let Tok::Identifier(var) = self.current_token.clone() {
            self.advance();
            self.eat(Tok::Assign);
            let start = self.expr();

            self.eat(Tok::To);
            let cond = self.expr();

            // Optional STEP expression
            let step = if self.current_token == Tok::Step {
                self.advance();
                self.expr()
            } else {
                Expr::Num(1)
            };

            Stmt::For {
                var,
                start: Box::new(start),
                cond: Box::new(cond),
                step: Box::new(step),
            }
        } else {
            panic!("Expected identifier, found: {:?}", self.current_token);
        }
    }

    fn next_stmt(&mut self) -> Stmt {
        if let Tok::Identifier(var) = self.current_token.clone() {
            self.advance();
            Stmt::Next { var }
        } else {
            panic!("Expected identifier, found: {:?}", self.current_token);
        }
    }

    fn stmt(&mut self) -> Stmt {
        match self.current_token {
            Tok::Let => {
                self.advance();
                self.let_stmt()
            }
            Tok::For => {
                self.advance();
                self.for_stmt()
            }
            Tok::Next => {
                self.advance();
                self.next_stmt()
            }
            Tok::Print => {
                self.advance();
                Stmt::Print {
                    ex: Box::new(self.expr()),
                }
            }
            Tok::End => {
                self.advance();
                Stmt::End
            }
            Tok::Rem(_) => {
                let comment = match self.current_token.clone() {
                    Tok::Rem(comment) => comment,
                    _ => panic!("Expected REM comment, found: {:?}", self.current_token),
                };
                self.advance();
                Stmt::Rem { comment }
            }
            _ => panic!(
                "Expected statement keyword:\nLET, FOR, PRINT...\nfound: {:?}",
                self.current_token
            ),
        }
    }

    fn prgrm(&mut self) -> Program {
        let mut lines = vec![];

        while self.current_token != Tok::Eof {
            lines.push(self.line());
            self.eat(Tok::Eol);
        }

        // sort by line numbers
        lines.sort_by(|a, b| a.line_number.cmp(&b.line_number));

        Program { lines }
    }
}
