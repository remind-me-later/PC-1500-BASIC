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
    Literal(i32),
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
            Expression::Literal(value) => write!(f, "{}", value),
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

pub enum Ast<'a> {
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
        to: Option<&'a mut Ast<'a>>,
    },
    End,
    GoSub {
        line_number: u32,
        to: Option<&'a mut Ast<'a>>,
    },
    Return,
    If {
        condition: Expression<'a>,
        then: &'a mut Ast<'a>,
        else_: Option<&'a mut Ast<'a>>,
    },
    Seq {
        statements: Vec<Ast<'a>>,
    },
    Program {
        lines: BTreeMap<u32, Ast<'a>>,
    },
}

impl std::fmt::Display for Ast<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ast::Let {
                variable,
                expression,
            } => {
                write!(f, "LET {} = {}", variable, expression)
            }
            Ast::Print { content } => {
                write!(f, "PRINT ")?;
                for (i, item) in content.iter().enumerate() {
                    if i > 0 {
                        write!(f, "; ")?;
                    }
                    write!(f, "{}", item)?;
                }
                Ok(())
            }
            Ast::Input { prompt, variable } => {
                write!(f, "INPUT ")?;
                if let Some(prompt) = prompt {
                    write!(f, "\"{}\"; ", prompt)?;
                }
                write!(f, "{}", variable)
            }
            Ast::Goto { line_number, .. } => write!(f, "GOTO {}", line_number),
            Ast::For {
                variable,
                from,
                to,
                step,
            } => {
                write!(f, "FOR {} = {} TO {}", variable, from, to)?;
                if let Some(step) = step {
                    write!(f, " STEP {}", step)?;
                }
                Ok(())
            }
            Ast::Next { variable } => write!(f, "NEXT {}", variable),
            Ast::End => write!(f, "END"),
            Ast::GoSub { line_number, .. } => write!(f, "GOSUB {}", line_number),
            Ast::Return => write!(f, "RETURN"),
            Ast::If {
                condition,
                then,
                else_,
            } => {
                write!(f, "IF {} THEN {}", condition, then)?;
                if let Some(else_) = else_ {
                    write!(f, " ELSE {}", else_)?;
                }
                Ok(())
            }
            Ast::Seq { statements } => {
                for (i, statement) in statements.iter().enumerate() {
                    if i > 0 {
                        write!(f, ": ")?;
                    }
                    write!(f, "{}", statement)?;
                }

                Ok(())
            }
            Ast::Program { lines } => {
                for (line_number, statement) in lines {
                    writeln!(f, "{} {}", line_number, statement)?;
                }

                Ok(())
            }
        }
    }
}

pub trait AstVisitor<'a> {
    // Expressions
    fn visit_literal(&mut self, num: i32);
    fn visit_variable(&mut self, variable: &'a str);
    fn visit_binary_op(
        &mut self,
        left: &Expression<'a>,
        op: BinaryOperator,
        right: &Expression<'a>,
    );

    // Statements
    fn visit_let(&mut self, variable: &'a str, expression: &Expression<'a>);
    fn visit_print(&mut self, content: &[PrintContent<'a>]);
    fn visit_input(&mut self, prompt: Option<&str>, variable: &'a str);
    fn visit_goto(&mut self, line_number: u32, to: Option<&'a Ast<'a>>);
    fn visit_for(
        &mut self,
        variable: &'a str,
        from: &Expression<'a>,
        to: &Expression<'a>,
        step: Option<&Expression<'a>>,
    );
    fn visit_next(&mut self, variable: &'a str);
    fn visit_end(&mut self);
    fn visit_gosub(&mut self, line_number: u32, to: Option<&'a Ast<'a>>);
    fn visit_return(&mut self);
    fn visit_if(
        &mut self,
        condition: &Expression<'a>,
        then: &'a Ast<'a>,
        else_: Option<&'a Ast<'a>>,
    );
    fn visit_seq(&mut self, statements: &'a [Ast<'a>]);
    fn visit_program(&mut self, lines: &'a BTreeMap<u32, Ast<'a>>);
}

pub trait MutAstVisitor<'a> {
    // Expressions
    fn visit_literal(&mut self, num: i32);
    fn visit_variable(&mut self, variable: &'a str);
    fn visit_binary_op(
        &mut self,
        left: &mut Expression<'a>,
        op: BinaryOperator,
        right: &mut Expression<'a>,
    );

    // Statements
    fn visit_let(&mut self, variable: &'a str, expression: &mut Expression<'a>);
    fn visit_print(&mut self, content: &mut Vec<PrintContent<'a>>);
    fn visit_input(&mut self, prompt: &mut Option<String>, variable: &'a str);
    fn visit_goto(&mut self, line_number: u32, to: &mut Option<&'a mut Ast<'a>>);
    fn visit_for(
        &mut self,
        variable: &'a str,
        from: &mut Expression<'a>,
        to: &mut Expression<'a>,
        step: &mut Option<Expression<'a>>,
    );
    fn visit_next(&mut self, variable: &'a str);
    fn visit_end(&mut self);
    fn visit_gosub(&mut self, line_number: u32, to: &mut Option<&'a mut Ast<'a>>);
    fn visit_return(&mut self);
    fn visit_if(
        &mut self,
        condition: &mut Expression<'a>,
        then: &mut Ast<'a>,
        else_: &mut Option<&'a mut Ast<'a>>,
    );
    fn visit_seq(&mut self, statements: &mut Vec<Ast<'a>>);
    fn visit_program(&mut self, lines: &mut BTreeMap<u32, Ast<'a>>);
}

impl<'a> Expression<'a> {
    pub fn accept<V: AstVisitor<'a>>(&self, visitor: &mut V) {
        match self {
            Expression::Literal(num) => visitor.visit_literal(*num),
            Expression::Variable(variable) => visitor.visit_variable(variable),
            Expression::BinaryOp { left, op, right } => visitor.visit_binary_op(left, *op, right),
        }
    }

    pub fn accept_mut<V: MutAstVisitor<'a>>(&mut self, visitor: &mut V) {
        match self {
            Expression::Literal(num) => visitor.visit_literal(*num),
            Expression::Variable(variable) => visitor.visit_variable(variable),
            Expression::BinaryOp { left, op, right } => {
                left.accept_mut(visitor);
                right.accept_mut(visitor);
                visitor.visit_binary_op(left, *op, right)
            }
        }
    }
}

impl<'a> Ast<'a> {
    pub fn accept<V: AstVisitor<'a>>(&'a self, visitor: &mut V) {
        match self {
            Ast::Let {
                variable,
                expression,
            } => visitor.visit_let(variable, expression),
            Ast::Print { content } => visitor.visit_print(content),
            Ast::Input { prompt, variable } => visitor.visit_input(prompt.as_deref(), variable),
            Ast::Goto { line_number, to } => visitor.visit_goto(*line_number, to.as_deref()),
            Ast::For {
                variable,
                from,
                to,
                step,
            } => visitor.visit_for(variable, from, to, step.as_ref()),
            Ast::Next { variable } => visitor.visit_next(variable),
            Ast::End => visitor.visit_end(),
            Ast::GoSub { line_number, to } => visitor.visit_gosub(*line_number, to.as_deref()),
            Ast::Return => visitor.visit_return(),
            Ast::If {
                condition,
                then,
                else_,
            } => visitor.visit_if(condition, then, else_.as_deref()),
            Ast::Seq { statements } => visitor.visit_seq(statements),
            Ast::Program { lines } => visitor.visit_program(lines),
        }
    }

    pub fn accept_mut<V: MutAstVisitor<'a>>(&'a mut self, visitor: &mut V) {
        match self {
            Ast::Let {
                variable,
                expression,
            } => visitor.visit_let(variable, expression),
            Ast::Print { content } => visitor.visit_print(content),
            Ast::Input { prompt, variable } => visitor.visit_input(prompt, variable),
            Ast::Goto { line_number, to } => visitor.visit_goto(*line_number, to),
            Ast::For {
                variable,
                from,
                to,
                step,
            } => visitor.visit_for(variable, from, to, step),
            Ast::Next { variable } => visitor.visit_next(variable),
            Ast::End => visitor.visit_end(),
            Ast::GoSub { line_number, to } => visitor.visit_gosub(*line_number, to),
            Ast::Return => visitor.visit_return(),
            Ast::If {
                condition,
                then,
                else_,
            } => visitor.visit_if(condition, then, else_),
            Ast::Seq { statements } => visitor.visit_seq(statements),
            Ast::Program { lines } => visitor.visit_program(lines),
        }
    }
}
