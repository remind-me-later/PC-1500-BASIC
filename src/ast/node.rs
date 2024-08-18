use std::collections::BTreeMap;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnaryOperator {
    Plus,
    Minus,
    Not,
}

impl std::fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOperator::Plus => write!(f, "+"),
            UnaryOperator::Minus => write!(f, "-"),
            UnaryOperator::Not => write!(f, "NOT"),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum LValue {
    Variable(String),
    ArrayElement {
        variable: String,
        index: Box<Expression>,
    },
}

impl std::fmt::Display for LValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LValue::Variable(variable) => write!(f, "{}", variable),
            LValue::ArrayElement { variable, index } => write!(f, "{}({})", variable, index),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Expression {
    Number(i32),
    String(String),
    LValue(LValue),
    Unary {
        op: UnaryOperator,
        operand: Box<Expression>,
    },
    Binary {
        left: Box<Expression>,
        op: BinaryOperator,
        right: Box<Expression>,
    },
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::String(content) => write!(f, "\"{}\"", content),
            Expression::Number(value) => write!(f, "{}", value),
            Expression::LValue(variable) => write!(f, "{}", variable),
            Expression::Unary { op, operand } => write!(f, "{}{}", op, operand),
            Expression::Binary { left, op, right } => write!(f, "{} {} {}", left, op, right),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DataItem {
    Number(i32),
    String(String),
}

#[derive(Debug)]
pub enum Statement {
    Let {
        variable: LValue,
        expression: Expression,
    },
    Dim {
        variable: String,
        size: u32,
        length: Option<u32>, // Only for strings
    },
    Print {
        content: Vec<Expression>,
    },
    Pause {
        content: Vec<Expression>,
    },
    Input {
        prompt: Option<Expression>,
        variable: LValue,
    },
    Wait {
        time: Option<Expression>,
    },
    Data {
        values: Vec<DataItem>,
    },
    Read {
        variables: Vec<LValue>,
    },
    Restore {
        line_number: Option<u32>,
    },
    Poke {
        address: u32,
        values: Vec<u8>,
    },
    Call {
        address: u32,
    },
    For {
        variable: String,
        from: Expression,
        to: Expression,
        step: Option<Expression>,
    },
    Next {
        variable: String,
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
        condition: Expression,
        then: Box<Statement>,
        else_: Option<Box<Statement>>,
    },
    Seq {
        statements: Vec<Statement>,
    },
    Rem {
        content: String,
    },
}

#[derive(Debug)]
pub struct Program {
    pub lines: BTreeMap<u32, Statement>,
}

impl Program {
    pub fn new() -> Self {
        Program {
            lines: BTreeMap::new(),
        }
    }

    pub fn add_line(&mut self, line_number: u32, statement: Statement) {
        self.lines.insert(line_number, statement);
    }

    pub fn lookup_line(&self, line_number: u32) -> Option<&Statement> {
        self.lines.get(&line_number)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&u32, &Statement)> {
        self.lines.iter()
    }

    pub fn values(&self) -> impl Iterator<Item = &Statement> {
        self.lines.values()
    }
}
