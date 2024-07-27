use std::collections::BTreeMap;

mod parser;
mod printer;
mod semantics;
mod visitor;

pub use parser::Parser;
pub use printer::Printer;
pub use semantics::SemanticChecker;
pub use visitor::{ExpressionVisitor, ProgramVisitor, StatementVisitor};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinaryOperator {
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    // Logical
    And,
    Or,
    // Comparison
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

impl std::fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Arithmetic
            BinaryOperator::Add => write!(f, "+"),
            BinaryOperator::Sub => write!(f, "-"),
            BinaryOperator::Mul => write!(f, "*"),
            BinaryOperator::Div => write!(f, "/"),
            // Logical
            BinaryOperator::And => write!(f, "AND"),
            BinaryOperator::Or => write!(f, "OR"),
            // Comparison
            BinaryOperator::Eq => write!(f, "="),
            BinaryOperator::Ne => write!(f, "<>"),
            BinaryOperator::Lt => write!(f, "<"),
            BinaryOperator::Le => write!(f, "<="),
            BinaryOperator::Gt => write!(f, ">"),
            BinaryOperator::Ge => write!(f, ">="),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Expression<'a> {
    NumberLiteral(i32),
    StringLiteral(&'a str),
    Variable(&'a str),
    Binary {
        left: &'a Expression<'a>,
        op: BinaryOperator,
        right: &'a Expression<'a>,
    },
}

impl std::fmt::Display for Expression<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::StringLiteral(content) => write!(f, "\"{}\"", content),
            Expression::NumberLiteral(value) => write!(f, "{}", value),
            Expression::Variable(variable) => write!(f, "{}", variable),
            Expression::Binary { left, op, right } => write!(f, "{} {} {}", left, op, right),
        }
    }
}

#[derive(Debug)]
pub enum Statement<'a> {
    Let {
        variable: &'a str,
        expression: &'a Expression<'a>,
    },
    Print {
        content: Vec<&'a Expression<'a>>,
    },
    Input {
        prompt: Option<&'a Expression<'a>>,
        variable: &'a str,
    },
    For {
        variable: &'a str,
        from: &'a Expression<'a>,
        to: &'a Expression<'a>,
        step: Option<&'a Expression<'a>>,
    },
    Next {
        variable: &'a str,
    },
    Goto {
        line_number: u32,
    },
    End,
    GoSub {
        line_number: u32,
    },
    Return,
    If {
        condition: &'a Expression<'a>,
        then: &'a Statement<'a>,
        else_: Option<&'a Statement<'a>>,
    },
    Seq {
        statements: Vec<Statement<'a>>,
    },
}

#[derive(Debug)]
pub struct Program<'a> {
    pub lines: BTreeMap<u32, Statement<'a>>,
}

impl<'a> Program<'a> {
    pub fn new() -> Self {
        Program {
            lines: BTreeMap::new(),
        }
    }

    pub fn add_line(&mut self, line_number: u32, ast: Statement<'a>) {
        self.lines.insert(line_number, ast);
    }

    pub fn lookup_line(&self, line_number: u32) -> Option<&Statement<'a>> {
        self.lines.get(&line_number)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&u32, &Statement<'a>)> {
        self.lines.iter()
    }

    pub fn values(&self) -> impl Iterator<Item = &Statement<'a>> {
        self.lines.values()
    }
}
