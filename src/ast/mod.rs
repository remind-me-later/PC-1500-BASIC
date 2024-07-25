use std::collections::BTreeMap;

mod ast_printer;
mod parser;
mod semantic_check;
mod symbol_table;

pub use ast_printer::AstPrintVisitor;
pub use parser::Parser;
pub use semantic_check::SemanticCheckVisitor;
pub use symbol_table::{SymbolTable, SymbolTableBuilderVisitor, Ty};

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

#[derive(PartialEq, Eq, Hash)]
pub enum Expression<'a> {
    NumberLiteral(i32),
    Variable(&'a str),
    BinaryOp {
        left: &'a Expression<'a>,
        op: BinaryOperator,
        right: &'a Expression<'a>,
    },
}

impl std::fmt::Display for Expression<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::NumberLiteral(value) => write!(f, "{}", value),
            Expression::Variable(variable) => write!(f, "{}", variable),
            Expression::BinaryOp { left, op, right } => write!(f, "{} {} {}", left, op, right),
        }
    }
}

impl std::fmt::Debug for Expression<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // print also direction
        match self {
            Expression::NumberLiteral(value) => {
                write!(f, "NumberLiteral({})({:p})", value, self)
            }
            Expression::Variable(variable) => write!(f, "Variable({})({:p})", variable, self),
            Expression::BinaryOp { left, op, right } => {
                write!(f, "BinaryOp({:?}, {:?}, {:?})({:p})", left, op, right, self)
            }
        }
    }
}

#[derive(Debug)]
pub enum PrintContent<'a> {
    StringLiteral(&'a str),
    Expression(&'a Expression<'a>),
}

impl std::fmt::Display for PrintContent<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrintContent::StringLiteral(content) => write!(f, "\"{}\"", content),
            PrintContent::Expression(expr) => write!(f, "{}", expr),
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
        content: Vec<PrintContent<'a>>,
    },
    Input {
        prompt: Option<&'a str>,
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
        then: &'a mut Statement<'a>,
        else_: Option<&'a mut Statement<'a>>,
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

pub trait ExpressionVisitor<'a, RetTy = ()> {
    fn visit_number_literal(&mut self, num: i32) -> RetTy;
    fn visit_variable(&mut self, variable: &'a str) -> RetTy;
    fn visit_binary_op(
        &mut self,
        left: &'a Expression<'a>,
        op: BinaryOperator,
        right: &'a Expression<'a>,
    ) -> RetTy;
}

impl<'a> Expression<'a> {
    pub fn accept<V: ExpressionVisitor<'a, RetTy>, RetTy>(&self, visitor: &mut V) -> RetTy {
        match self {
            Expression::NumberLiteral(num) => visitor.visit_number_literal(*num),
            Expression::Variable(variable) => visitor.visit_variable(variable),
            Expression::BinaryOp { left, op, right } => visitor.visit_binary_op(left, *op, right),
        }
    }
}

pub trait StatementVisitor<'a> {
    fn visit_let(&mut self, variable: &'a str, expression: &Expression<'a>);
    fn visit_print(&mut self, content: &[PrintContent<'a>]);
    fn visit_input(&mut self, prompt: Option<&str>, variable: &'a str);
    fn visit_goto(&mut self, line_number: u32);
    fn visit_for(
        &mut self,
        variable: &'a str,
        from: &'a Expression<'a>,
        to: &'a Expression<'a>,
        step: Option<&'a Expression<'a>>,
    );
    fn visit_next(&mut self, variable: &'a str);
    fn visit_end(&mut self);
    fn visit_gosub(&mut self, line_number: u32);
    fn visit_return(&mut self);
    fn visit_if(
        &mut self,
        condition: &Expression<'a>,
        then: &'a Statement<'a>,
        else_: Option<&'a Statement<'a>>,
    );
    fn visit_seq(&mut self, statements: &'a [Statement<'a>]);
}

impl<'a> Statement<'a> {
    pub fn accept<V: StatementVisitor<'a>>(&'a self, visitor: &mut V) {
        match self {
            Statement::Let {
                variable,
                expression,
            } => visitor.visit_let(variable, expression),
            Statement::Print { content } => visitor.visit_print(content),
            Statement::Input { prompt, variable } => {
                visitor.visit_input(prompt.as_deref(), variable)
            }
            Statement::Goto { line_number } => visitor.visit_goto(*line_number),
            Statement::For {
                variable,
                from,
                to,
                step,
            } => visitor.visit_for(variable, from, to, *step),
            Statement::Next { variable } => visitor.visit_next(variable),
            Statement::End => visitor.visit_end(),
            Statement::GoSub { line_number } => visitor.visit_gosub(*line_number),
            Statement::Return => visitor.visit_return(),
            Statement::If {
                condition,
                then,
                else_,
            } => visitor.visit_if(condition, then, else_.as_deref()),
            Statement::Seq { statements } => visitor.visit_seq(statements),
        }
    }
}

pub trait ProgramVisitor<'a> {
    fn visit_program(&mut self, program: &'a Program<'a>);
}

impl<'a> Program<'a> {
    pub fn accept<V: ProgramVisitor<'a>>(&'a self, visitor: &mut V) {
        visitor.visit_program(self);
    }
}
