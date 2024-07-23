use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

pub enum Expression<'a> {
    NumberLiteral(i32),
    Variable(&'a str),
    BinaryOp {
        left: &'a mut Expression<'a>,
        op: BinaryOperator,
        right: &'a mut Expression<'a>,
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

pub enum PrintContent<'a> {
    StringLiteral(String),
    Expression(Expression<'a>),
}

impl std::fmt::Display for PrintContent<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrintContent::StringLiteral(content) => write!(f, "\"{}\"", content),
            PrintContent::Expression(expr) => write!(f, "{}", expr),
        }
    }
}

pub enum Statement<'a> {
    Let {
        variable: &'a str,
        expression: Expression<'a>,
    },
    Print {
        content: Vec<PrintContent<'a>>,
    },
    Input {
        prompt: Option<String>,
        variable: &'a str,
    },
    For {
        variable: &'a str,
        from: Expression<'a>,
        to: Expression<'a>,
        step: Option<Expression<'a>>,
    },
    Next {
        variable: &'a str,
    },
    Goto {
        line_number: u32,
        to: Option<&'a mut Statement<'a>>,
    },
    End,
    GoSub {
        line_number: u32,
        to: Option<&'a mut Statement<'a>>,
    },
    Return,
    If {
        condition: Expression<'a>,
        then: &'a mut Statement<'a>,
        else_: Option<&'a mut Statement<'a>>,
    },
    Seq {
        statements: Vec<Statement<'a>>,
    },
}

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
        left: &Expression<'a>,
        op: BinaryOperator,
        right: &Expression<'a>,
    ) -> RetTy;
}

pub trait MutExpressionVisitor<'a, RetTy = ()> {
    fn visit_number_literal(&mut self, num: i32) -> RetTy;
    fn visit_variable(&mut self, variable: &'a str) -> RetTy;
    fn visit_binary_op(
        &mut self,
        left: &mut Expression<'a>,
        op: BinaryOperator,
        right: &mut Expression<'a>,
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

    pub fn accept_mut<V: MutExpressionVisitor<'a, RetTy>, RetTy>(
        &mut self,
        visitor: &mut V,
    ) -> RetTy {
        match self {
            Expression::NumberLiteral(num) => visitor.visit_number_literal(*num),
            Expression::Variable(variable) => visitor.visit_variable(variable),
            Expression::BinaryOp { left, op, right } => {
                left.accept_mut(visitor);
                right.accept_mut(visitor);
                visitor.visit_binary_op(left, *op, right)
            }
        }
    }
}

pub trait StatementVisitor<'a> {
    fn visit_let(&mut self, variable: &'a str, expression: &Expression<'a>);
    fn visit_print(&mut self, content: &[PrintContent<'a>]);
    fn visit_input(&mut self, prompt: Option<&str>, variable: &'a str);
    fn visit_goto(&mut self, line_number: u32, to: Option<&'a Statement<'a>>);
    fn visit_for(
        &mut self,
        variable: &'a str,
        from: &Expression<'a>,
        to: &Expression<'a>,
        step: Option<&Expression<'a>>,
    );
    fn visit_next(&mut self, variable: &'a str);
    fn visit_end(&mut self);
    fn visit_gosub(&mut self, line_number: u32, to: Option<&'a Statement<'a>>);
    fn visit_return(&mut self);
    fn visit_if(
        &mut self,
        condition: &Expression<'a>,
        then: &'a Statement<'a>,
        else_: Option<&'a Statement<'a>>,
    );
    fn visit_seq(&mut self, statements: &'a [Statement<'a>]);
}

pub trait MutStatementVisitor<'a> {
    fn visit_let(&mut self, variable: &'a str, expression: &mut Expression<'a>);
    fn visit_print(&mut self, content: &mut Vec<PrintContent<'a>>);
    fn visit_input(&mut self, prompt: &mut Option<String>, variable: &'a str);
    fn visit_goto(&mut self, line_number: u32, to: &mut Option<&'a mut Statement<'a>>);
    fn visit_for(
        &mut self,
        variable: &'a str,
        from: &mut Expression<'a>,
        to: &mut Expression<'a>,
        step: &mut Option<Expression<'a>>,
    );
    fn visit_next(&mut self, variable: &'a str);
    fn visit_end(&mut self);
    fn visit_gosub(&mut self, line_number: u32, to: &mut Option<&'a mut Statement<'a>>);
    fn visit_return(&mut self);
    fn visit_if(
        &mut self,
        condition: &mut Expression<'a>,
        then: &mut Statement<'a>,
        else_: &mut Option<&'a mut Statement<'a>>,
    );
    fn visit_seq(&mut self, statements: &mut Vec<Statement<'a>>);
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
            Statement::Goto { line_number, to } => visitor.visit_goto(*line_number, to.as_deref()),
            Statement::For {
                variable,
                from,
                to,
                step,
            } => visitor.visit_for(variable, from, to, step.as_ref()),
            Statement::Next { variable } => visitor.visit_next(variable),
            Statement::End => visitor.visit_end(),
            Statement::GoSub { line_number, to } => {
                visitor.visit_gosub(*line_number, to.as_deref())
            }
            Statement::Return => visitor.visit_return(),
            Statement::If {
                condition,
                then,
                else_,
            } => visitor.visit_if(condition, then, else_.as_deref()),
            Statement::Seq { statements } => visitor.visit_seq(statements),
        }
    }

    pub fn accept_mut<V: MutStatementVisitor<'a>>(&'a mut self, visitor: &mut V) {
        match self {
            Statement::Let {
                variable,
                expression,
            } => visitor.visit_let(variable, expression),
            Statement::Print { content } => visitor.visit_print(content),
            Statement::Input { prompt, variable } => visitor.visit_input(prompt, variable),
            Statement::Goto { line_number, to } => visitor.visit_goto(*line_number, to),
            Statement::For {
                variable,
                from,
                to,
                step,
            } => visitor.visit_for(variable, from, to, step),
            Statement::Next { variable } => visitor.visit_next(variable),
            Statement::End => visitor.visit_end(),
            Statement::GoSub { line_number, to } => visitor.visit_gosub(*line_number, to),
            Statement::Return => visitor.visit_return(),
            Statement::If {
                condition,
                then,
                else_,
            } => visitor.visit_if(condition, then, else_),
            Statement::Seq { statements } => visitor.visit_seq(statements),
        }
    }
}

pub trait ProgramVisitor<'a> {
    fn visit_program(&mut self, program: &'a Program<'a>);
}

pub trait MutProgramVisitor<'a> {
    fn visit_program(&mut self, program: &'a mut Program<'a>);
}

impl<'a> Program<'a> {
    pub fn accept<V: ProgramVisitor<'a>>(&'a self, visitor: &mut V) {
        visitor.visit_program(self);
    }

    pub fn accept_mut<V: MutProgramVisitor<'a>>(&'a mut self, visitor: &mut V) {
        visitor.visit_program(self);
    }
}
