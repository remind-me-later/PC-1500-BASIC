use core::panic;
use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    rc::Rc,
};

use super::lexer::{Lexer, Tok};

#[derive(Debug, Clone)]
pub enum Stmt {
    Let {
        var: Rc<RefCell<Var>>,
        ex: Aexpr,
    },
    If {
        cond: Bexpr,
        then: Rc<RefCell<Stmt>>,
        else_: Option<Rc<RefCell<Stmt>>>,
    },
    For {
        var: Rc<RefCell<Var>>,
        start: Aexpr,
        cond: Aexpr,
        step: Aexpr,
        body: Vec<Rc<RefCell<Stmt>>>,
    },
    Next {
        var: Rc<RefCell<Var>>,
    },
    Input {
        prompt: Option<String>,
        var: Rc<RefCell<Var>>,
    },
    Print {
        exs: Vec<Aexpr>,
    },
    End,
    Rem,
    Goto {
        line_number: u64,
        to: Option<Rc<RefCell<Stmt>>>,
    },
    Gosub {
        line_number: u64,
        to: Option<Rc<RefCell<Stmt>>>,
    },
    Return,
    Concat {
        lhs: Rc<RefCell<Stmt>>,
        rhs: Rc<RefCell<Stmt>>,
    },
}

impl std::fmt::Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::Let { var, ex } => write!(f, "LET {} = {}", var.borrow().id, ex),
            Stmt::For {
                var,
                start,
                cond,
                step,
                body,
            } => {
                write!(
                    f,
                    "FOR {} = {} TO {} STEP {}",
                    var.borrow(),
                    start,
                    cond,
                    step
                );

                for stmt in body {
                    write!(f, "\n{}", stmt.borrow())?;
                }

                write!(f, "\nNEXT {}", var.borrow())?;

                Ok(())
            }
            Stmt::If { cond, then, else_ } => {
                write!(f, "IF {} THEN {}", cond, then.borrow())?;
                if let Some(else_) = else_ {
                    write!(f, " ELSE {}", else_.borrow())?;
                }
                Ok(())
            }
            Stmt::Next { var } => write!(f, "NEXT {}", var.borrow()),
            Stmt::Input { prompt, var } => {
                if let Some(prompt) = prompt {
                    write!(f, "INPUT \"{}\"; {}", prompt, var.borrow())
                } else {
                    write!(f, "INPUT {}", var.borrow())
                }
            }
            Stmt::Print { exs } => {
                write!(f, "PRINT ")?;
                for (i, ex) in exs.iter().enumerate() {
                    write!(f, "{}", ex)?;
                    if i < exs.len() - 1 {
                        write!(f, "; ")?;
                    }
                }
                Ok(())
            }
            Stmt::End => write!(f, "END"),
            Stmt::Rem => write!(f, "REM"),
            Stmt::Goto { line_number, .. } => write!(f, "GOTO {}", line_number),
            Stmt::Gosub { line_number, .. } => write!(f, "GOSUB {}", line_number),
            Stmt::Return => write!(f, "RETURN"),
            Stmt::Concat { lhs, rhs } => write!(f, "{}: {}", lhs.borrow(), rhs.borrow()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum AexprOp {
    Plus,
    Minus,
    Star,
    Slash,
}

impl From<Tok> for AexprOp {
    fn from(tok: Tok) -> Self {
        match tok {
            Tok::Plus => AexprOp::Plus,
            Tok::Minus => AexprOp::Minus,
            Tok::Star => AexprOp::Star,
            Tok::Slash => AexprOp::Slash,
            _ => panic!("Unexpected token in AexprOp"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Aexpr {
    ArithOp {
        op: AexprOp,
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
                    AexprOp::Plus => "+",
                    AexprOp::Minus => "-",
                    AexprOp::Star => "*",
                    AexprOp::Slash => "/",
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
        lhs: Aexpr,
        rhs: Aexpr,
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

#[derive(Debug, Clone, Copy)]
pub enum VarTy {
    Num,
    Str,
}

#[derive(Debug, Clone)]
pub struct Var {
    id: String,
    ty: VarTy,
}

impl Var {
    pub fn new(id: String) -> Self {
        let ty = if id.ends_with('$') {
            VarTy::Str
        } else {
            VarTy::Num
        };

        Var { id, ty }
    }
}

impl std::fmt::Display for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

pub struct Parser {
    lexer: Lexer,
    current_token: Tok,
    vars: HashMap<String, Rc<RefCell<Var>>>,
    lines: BTreeMap<u64, Rc<RefCell<Stmt>>>,
    dangling_gotos: Vec<(u64, Rc<RefCell<Stmt>>)>,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next_token();
        Parser {
            lexer,
            current_token,
            vars: HashMap::new(),
            lines: BTreeMap::new(),
            dangling_gotos: Vec::new(),
        }
    }

    pub fn parse(&mut self) {
        self.prgrm()
    }

    // Getters
    pub fn get_vars(&self) -> &HashMap<String, Rc<RefCell<Var>>> {
        &self.vars
    }

    pub fn get_lines(&self) -> &BTreeMap<u64, Rc<RefCell<Stmt>>> {
        &self.lines
    }

    // Linking
    fn link_var(&mut self, id: String) -> Rc<RefCell<Var>> {
        if let Some(var) = self.vars.get(&id) {
            Rc::clone(var)
        } else {
            let var = Rc::new(RefCell::new(Var::new(id.clone())));
            self.vars.insert(id, Rc::clone(&var));
            var
        }
    }

    fn try_link_goto(&mut self, line_number: u64) -> Option<Rc<RefCell<Stmt>>> {
        if let Some(stmt) = self.lines.get(&line_number) {
            Some(Rc::clone(stmt))
        } else {
            self.dangling_gotos.push((
                line_number,
                Rc::clone(self.lines.get(&line_number).unwrap()),
            ));
            None
        }
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
            let op = AexprOp::from(token);

            self.advance();
            node = Aexpr::ArithOp {
                lhs: Box::new(node),
                op,
                rhs: Box::new(self.factor()),
            };
        }
        node
    }

    fn expr(&mut self) -> Aexpr {
        let mut node = self.term();
        while self.current_token == Tok::Plus || self.current_token == Tok::Minus {
            let token = self.current_token.clone();
            let op = AexprOp::from(token);
            self.advance();
            node = Aexpr::ArithOp {
                lhs: Box::new(node),
                op,
                rhs: Box::new(self.term()),
            };
        }
        node
    }

    fn bexpr(&mut self) -> Bexpr {
        let lhs = self.expr();
        let node = if self.current_token == Tok::LessThan
            || self.current_token == Tok::GreaterThan
            || self.current_token == Tok::Diamond
            || self.current_token == Tok::Eq
        {
            let op = self.current_token.clone();
            self.advance();
            let rhs = self.expr();
            Bexpr::ArithOp { lhs, op, rhs }
        } else {
            panic!(
                "Expected relational operator, found: {:?}",
                self.current_token
            );
        };

        if self.current_token == Tok::And || self.current_token == Tok::Or {
            let op = self.current_token.clone();
            self.advance();
            Bexpr::BoolOp {
                lhs: Box::new(node),
                op,
                rhs: Box::new(self.bexpr()),
            }
        } else {
            node
        }
    }

    // Line parsing functions
    fn line(&mut self) -> (u64, Rc<RefCell<Stmt>>) {
        let line_number = match self.current_token.clone() {
            Tok::Number(value) => {
                self.advance();
                value
            }
            _ => panic!("Expected line number, found: {:?}", self.current_token),
        };

        let stmt = self.stmt();

        (line_number, stmt)
    }

    // Line statement parsing functions
    fn let_stmt(&mut self) -> Rc<RefCell<Stmt>> {
        if let Tok::Identifier(id) = self.current_token.clone() {
            self.advance();
            self.eat(Tok::Eq);
            let var = self.link_var(id);
            let ex = self.expr();
            Rc::new(RefCell::new(Stmt::Let { var, ex }))
        } else {
            panic!("Expected identifier, found: {:?}", self.current_token);
        }
    }

    fn for_stmt(&mut self) -> Rc<RefCell<Stmt>> {
        if let Tok::Identifier(id) = self.current_token.clone() {
            let var = self.link_var(id);
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

            Rc::new(RefCell::new(Stmt::For {
                var,
                start,
                cond,
                step,
                body: Vec::new(),
            }))
        } else {
            panic!("Expected identifier, found: {:?}", self.current_token);
        }
    }

    fn next_stmt(&mut self) -> Rc<RefCell<Stmt>> {
        if let Tok::Identifier(id) = self.current_token.clone() {
            self.advance();
            let var = self.link_var(id);

            Rc::new(RefCell::new(Stmt::Next { var }))
        } else {
            panic!("Expected identifier, found: {:?}", self.current_token);
        }
    }

    fn goto_stmt(&mut self) -> Rc<RefCell<Stmt>> {
        if let Tok::Number(line_number) = self.current_token.clone() {
            self.advance();
            let to = self.try_link_goto(line_number);
            Rc::new(RefCell::new(Stmt::Goto { line_number, to }))
        } else {
            panic!("Expected line number, found: {:?}", self.current_token);
        }
    }

    fn gosub_stmt(&mut self) -> Rc<RefCell<Stmt>> {
        if let Tok::Number(line_number) = self.current_token.clone() {
            self.advance();
            let to = self.try_link_goto(line_number);
            Rc::new(RefCell::new(Stmt::Gosub { line_number, to }))
        } else {
            panic!("Expected line number, found: {:?}", self.current_token);
        }
    }

    fn print_stmt(&mut self) -> Rc<RefCell<Stmt>> {
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
        Rc::new(RefCell::new(Stmt::Print { exs }))
    }

    fn input_stmt(&mut self) -> Rc<RefCell<Stmt>> {
        match self.current_token.clone() {
            Tok::StringLiteral(prompt) => {
                self.advance();
                self.eat(Tok::SemiColon);
                if let Tok::Identifier(id) = self.current_token.clone() {
                    self.advance();
                    let input = Stmt::Input {
                        prompt: Some(prompt),
                        var: self.link_var(id),
                    };
                    Rc::new(RefCell::new(input))
                } else {
                    panic!("Expected identifier, found: {:?}", self.current_token);
                }
            }
            Tok::Identifier(id) => {
                self.advance();
                let input = Stmt::Input {
                    prompt: None,
                    var: self.link_var(id),
                };
                Rc::new(RefCell::new(input))
            }
            _ => panic!(
                "Expected string literal or identifier, found: {:?}",
                self.current_token
            ),
        }
    }

    fn if_stmt(&mut self) -> Rc<RefCell<Stmt>> {
        let cond = self.bexpr();
        // THEN keyword is optional
        if self.current_token == Tok::Then {
            self.advance();
        }

        let then = self.stmt();
        let else_ = if self.current_token == Tok::Else {
            self.advance();
            Some(self.stmt())
        } else {
            None
        };

        Rc::new(RefCell::new(Stmt::If { cond, then, else_ }))
    }

    fn atomic_stmt(&mut self) -> Rc<RefCell<Stmt>> {
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
                Rc::new(RefCell::new(Stmt::End))
            }
            Tok::Rem => {
                self.advance();
                Rc::new(RefCell::new(Stmt::Rem))
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
                Rc::new(RefCell::new(Stmt::Return))
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

    fn stmt(&mut self) -> Rc<RefCell<Stmt>> {
        let lhs = self.atomic_stmt();

        if self.current_token != Tok::Colon {
            return lhs;
        }

        // TODO: check for EOL?

        self.eat(Tok::Colon);
        let rhs = self.stmt();

        let stmt = Stmt::Concat { lhs, rhs };
        Rc::new(RefCell::new(stmt))
    }

    fn prgrm(&mut self) {
        loop {
            while self.current_token == Tok::Eol {
                self.advance();
            }

            if self.current_token == Tok::Eof {
                break;
            }

            let (line_number, stmt) = self.line();

            // Ignore comments
            if let Stmt::Rem { .. } = &*stmt.borrow() {
                continue;
            }

            // Check for duplicate line numbers
            if self.lines.insert(line_number, stmt).is_some() {
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

        // Link dangling GOTO and GOSUB statements
        while !self.dangling_gotos.is_empty() {
            if let Some((line_number, stmt)) = self.dangling_gotos.pop() {
                match &mut *stmt.borrow_mut() {
                    Stmt::Goto { to, .. } => {
                        *to = self.try_link_goto(line_number);
                        if to.is_none() {
                            panic!("Unresolved GOTO statement: {}", line_number);
                        }
                    }
                    Stmt::Gosub { to, .. } => {
                        *to = self.try_link_goto(line_number);
                        if to.is_none() {
                            panic!("Unresolved GOSUB statement: {}", line_number);
                        }
                    }
                    _ => panic!("Expected GOTO or GOSUB statement"),
                }
            }
        }
    }
}
