use std::collections::BTreeSet;

use super::lexer::{Lexer, Tok};

#[derive(Debug, Clone)]
pub struct ParsedLines {
    lines: BTreeSet<Line>,
}

impl IntoIterator for ParsedLines {
    type Item = Line;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.lines.into_iter().collect::<Vec<Line>>().into_iter()
    }
}

impl std::fmt::Display for ParsedLines {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Don't add newline in last line
        for line in self.lines.iter().take(self.lines.len() - 1) {
            writeln!(f, "{}", line)?;
        }
        if let Some(last_line) = self.lines.iter().last() {
            write!(f, "{}", last_line)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Line {
    number: u64,
    stmt: LineStmt,
}

impl std::cmp::PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number
    }
}

impl std::cmp::Eq for Line {}

impl std::cmp::PartialOrd for Line {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for Line {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.number.cmp(&other.number)
    }
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.number, self.stmt)
    }
}

#[derive(Debug, Clone)]
pub enum LineStmt {
    Let {
        var: String,
        ex: Box<Aexpr>,
    },
    For {
        var: String,
        start: Box<Aexpr>,
        cond: Box<Aexpr>,
        step: Box<Aexpr>,
    },
    If {
        cond: Box<Bexpr>,
        then: Box<LineStmt>,
        else_: Option<Box<LineStmt>>,
    },
    Next {
        var: String,
    },
    Input {
        prompt: Option<String>,
        var: String,
    },
    Print {
        exs: Vec<Aexpr>,
    },
    End,
    Rem {
        comment: String,
    },
    Goto {
        line_number: u64,
    },
    Gosub {
        line_number: u64,
    },
    Return,
    Concat {
        lhs: Box<LineStmt>,
        rhs: Box<LineStmt>,
    },
}

impl std::fmt::Display for LineStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineStmt::Let { var, ex } => write!(f, "LET {} = {}", var, ex),
            LineStmt::For {
                var,
                start,
                cond,
                step,
            } => {
                write!(f, "FOR {} = {} TO {} STEP {}", var, start, cond, step)
            }
            LineStmt::If { cond, then, else_ } => {
                write!(f, "IF {} THEN {}", cond, then)?;
                if let Some(else_) = else_ {
                    write!(f, " ELSE {}", else_)?;
                }
                Ok(())
            }
            LineStmt::Next { var } => write!(f, "NEXT {}", var),
            LineStmt::Input { prompt, var } => {
                if let Some(prompt) = prompt {
                    write!(f, "INPUT \"{}\"; {}", prompt, var)
                } else {
                    write!(f, "INPUT {}", var)
                }
            }
            LineStmt::Print { exs } => {
                write!(f, "PRINT ")?;
                for (i, ex) in exs.iter().enumerate() {
                    write!(f, "{}", ex)?;
                    if i < exs.len() - 1 {
                        write!(f, "; ")?;
                    }
                }
                Ok(())
            }
            LineStmt::End => write!(f, "END"),
            LineStmt::Rem { comment } => write!(f, "REM {}", comment),
            LineStmt::Goto { line_number } => write!(f, "GOTO {}", line_number),
            LineStmt::Gosub { line_number } => write!(f, "GOSUB {}", line_number),
            LineStmt::Return => write!(f, "RETURN"),
            LineStmt::Concat { lhs, rhs } => write!(f, "{}: {}", lhs, rhs),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Aexpr {
    ArithOp {
        op: Tok,
        lhs: Box<Aexpr>,
        rhs: Box<Aexpr>,
    },
    Num(u64),
    StringLiteral(String),
    Var(String),
}

impl std::fmt::Display for Aexpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Aexpr::ArithOp { lhs, op, rhs } => {
                let op_str = match op {
                    Tok::Plus => "+",
                    Tok::Minus => "-",
                    Tok::Star => "*",
                    Tok::Slash => "/",
                    _ => panic!("Unexpected token in BinOp"),
                };
                write!(f, "({} {} {})", lhs, op_str, rhs)
            }
            Aexpr::Num(value) => write!(f, "{}", value),
            Aexpr::Var(name) => write!(f, "{}", name),
            Aexpr::StringLiteral(value) => write!(f, "\"{}\"", value),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Bexpr {
    ArithOp {
        op: Tok,
        lhs: Box<Aexpr>,
        rhs: Box<Aexpr>,
    },
    BoolOp {
        op: Tok,
        lhs: Box<Bexpr>,
        rhs: Box<Bexpr>,
    },
}

impl std::fmt::Display for Bexpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Bexpr::ArithOp { lhs, op, rhs } => {
                let op_str = match op {
                    Tok::Diamond => "<>",
                    Tok::GreaterThan => ">",
                    Tok::LessThan => "<",
                    Tok::Eq => "=",
                    _ => panic!("Unexpected token in BinOp"),
                };
                write!(f, "({} {} {})", lhs, op_str, rhs)
            }
            Bexpr::BoolOp { lhs, op, rhs } => {
                let op_str = match op {
                    Tok::And => "AND",
                    Tok::Or => "OR",
                    _ => panic!("Unexpected token in BinOp"),
                };
                write!(f, "({} {} {})", lhs, op_str, rhs)
            }
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

    pub fn parse(&mut self) -> ParsedLines {
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
    fn factor(&mut self) -> Aexpr {
        match self.current_token.clone() {
            Tok::Number(value) => {
                self.advance();
                Aexpr::Num(value)
            }
            Tok::StringLiteral(value) => {
                self.advance();
                Aexpr::StringLiteral(value)
            }
            Tok::Identifier(name) => {
                self.advance();
                Aexpr::Var(name)
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

    fn term(&mut self) -> Aexpr {
        let mut node = self.factor();
        while self.current_token == Tok::Star || self.current_token == Tok::Slash {
            let token = self.current_token.clone();
            self.advance();
            node = Aexpr::ArithOp {
                lhs: Box::new(node),
                op: token,
                rhs: Box::new(self.factor()),
            };
        }
        node
    }

    fn expr(&mut self) -> Aexpr {
        let mut node = self.term();
        while self.current_token == Tok::Plus || self.current_token == Tok::Minus {
            let token = self.current_token.clone();
            self.advance();
            node = Aexpr::ArithOp {
                lhs: Box::new(node),
                op: token,
                rhs: Box::new(self.term()),
            };
        }
        node
    }

    fn bexpr(&mut self) -> Bexpr {
        let anode = self.expr();
        let node = if self.current_token == Tok::LessThan
            || self.current_token == Tok::GreaterThan
            || self.current_token == Tok::Diamond
            || self.current_token == Tok::Eq
        {
            let token = self.current_token.clone();
            self.advance();
            Bexpr::ArithOp {
                lhs: Box::new(anode),
                op: token,
                rhs: Box::new(self.expr()),
            }
        } else {
            panic!(
                "Expected relational operator, found: {:?}",
                self.current_token
            );
        };

        if self.current_token == Tok::And || self.current_token == Tok::Or {
            let token = self.current_token.clone();
            self.advance();
            Bexpr::BoolOp {
                lhs: Box::new(node),
                op: token,
                rhs: Box::new(self.bexpr()),
            }
        } else {
            node
        }
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

        Line {
            number: line_number,
            stmt,
        }
    }

    // Line statement parsing functions
    fn let_stmt(&mut self) -> LineStmt {
        if let Tok::Identifier(var) = self.current_token.clone() {
            self.advance();
            self.eat(Tok::Eq);
            LineStmt::Let {
                var,
                ex: Box::new(self.expr()),
            }
        } else {
            panic!("Expected identifier, found: {:?}", self.current_token);
        }
    }

    fn for_stmt(&mut self) -> LineStmt {
        if let Tok::Identifier(var) = self.current_token.clone() {
            self.advance();
            self.eat(Tok::Eq);
            let start = self.expr();

            self.eat(Tok::To);
            let cond = self.expr();

            // Optional STEP expression
            let step = if self.current_token == Tok::Step {
                self.advance();
                self.expr()
            } else {
                Aexpr::Num(1)
            };

            LineStmt::For {
                var,
                start: Box::new(start),
                cond: Box::new(cond),
                step: Box::new(step),
            }
        } else {
            panic!("Expected identifier, found: {:?}", self.current_token);
        }
    }

    fn next_stmt(&mut self) -> LineStmt {
        if let Tok::Identifier(var) = self.current_token.clone() {
            self.advance();
            LineStmt::Next { var }
        } else {
            panic!("Expected identifier, found: {:?}", self.current_token);
        }
    }

    fn goto_stmt(&mut self) -> LineStmt {
        if let Tok::Number(line_number) = self.current_token.clone() {
            self.advance();
            LineStmt::Goto { line_number }
        } else {
            panic!("Expected line number, found: {:?}", self.current_token);
        }
    }

    fn gosub_stmt(&mut self) -> LineStmt {
        if let Tok::Number(line_number) = self.current_token.clone() {
            self.advance();
            LineStmt::Gosub { line_number }
        } else {
            panic!("Expected line number, found: {:?}", self.current_token);
        }
    }

    fn print_stmt(&mut self) -> LineStmt {
        // TODO: is an empty PRINT statement valid?
        let mut exs = Vec::new();
        loop {
            exs.push(self.expr());
            if self.current_token == Tok::SemiColon {
                self.advance();
            } else {
                break;
            }
        }
        LineStmt::Print { exs }
    }

    fn input_stmt(&mut self) -> LineStmt {
        match self.current_token.clone() {
            Tok::StringLiteral(prompt) => {
                self.advance();
                self.eat(Tok::SemiColon);
                if let Tok::Identifier(var) = self.current_token.clone() {
                    self.advance();
                    LineStmt::Input {
                        prompt: Some(prompt),
                        var,
                    }
                } else {
                    panic!("Expected identifier, found: {:?}", self.current_token);
                }
            }
            Tok::Identifier(var) => {
                self.advance();
                LineStmt::Input { prompt: None, var }
            }
            _ => panic!(
                "Expected string literal or identifier, found: {:?}",
                self.current_token
            ),
        }
    }

    fn if_stmt(&mut self) -> LineStmt {
        let cond = self.bexpr();
        self.eat(Tok::Then);
        let then = Box::new(self.stmt());
        let else_ = if self.current_token == Tok::Else {
            self.advance();
            Some(Box::new(self.stmt()))
        } else {
            None
        };
        LineStmt::If {
            cond: Box::new(cond),
            then,
            else_,
        }
    }

    fn atomic_stmt(&mut self) -> LineStmt {
        match self.current_token {
            Tok::Let => {
                self.advance();
                self.let_stmt()
            }
            Tok::Identifier(_) => self.let_stmt(),
            Tok::For => {
                self.advance();
                self.for_stmt()
            }
            Tok::Next => {
                self.advance();
                self.next_stmt()
            }
            Tok::Input => {
                self.advance();
                self.input_stmt()
            }
            Tok::Print => {
                self.advance();
                self.print_stmt()
            }
            Tok::End => {
                self.advance();
                LineStmt::End
            }
            Tok::Rem(_) => {
                let comment = match self.current_token.clone() {
                    Tok::Rem(comment) => comment,
                    _ => panic!("Expected REM comment, found: {:?}", self.current_token),
                };
                self.advance();
                LineStmt::Rem { comment }
            }
            Tok::Goto => {
                self.advance();
                self.goto_stmt()
            }
            Tok::Gosub => {
                self.advance();
                self.gosub_stmt()
            }
            Tok::Return => {
                self.advance();
                LineStmt::Return
            }
            Tok::If => {
                self.advance();
                self.if_stmt()
            }
            _ => panic!(
                "Expected statement keyword:\nLET, FOR, PRINT...\nfound: {:?}",
                self.current_token
            ),
        }
    }

    fn stmt(&mut self) -> LineStmt {
        let lhs = self.atomic_stmt();

        if self.current_token != Tok::Colon {
            return lhs;
        }

        // TODO: check for EOL?

        self.eat(Tok::Colon);
        let rhs = self.stmt();
        LineStmt::Concat {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    fn prgrm(&mut self) -> ParsedLines {
        let mut lines = BTreeSet::new();

        loop {
            while self.current_token == Tok::Eol {
                self.advance();
            }

            if self.current_token == Tok::Eof {
                break;
            }

            let line = self.line();
            let line_number = line.number;

            // Check for duplicate line numbers
            if !lines.insert(line) {
                panic!("Duplicate line number: {}", line_number);
            }

            // Check for EOL or EOF
            if self.current_token == Tok::Eol {
                self.advance();
            } else if self.current_token == Tok::Eof {
                break;
            } else {
                panic!("Expected EOL or EOF, found: {:?}", self.current_token);
            }
        }

        ParsedLines { lines }
    }
}
