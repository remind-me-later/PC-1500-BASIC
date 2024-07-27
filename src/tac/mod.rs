mod builder;

pub use builder::Builder;

use crate::ast;

pub const START_LABEL: u32 = 0;
pub const BEGIN_OF_BUILTIN_LABELS: u32 = 1;
pub const END_OF_BUILTIN_LABELS: u32 = 20;
pub const PRINT_PTR_LABEL: u32 = BEGIN_OF_BUILTIN_LABELS;
pub const INPUT_PTR_LABEL: u32 = BEGIN_OF_BUILTIN_LABELS + 1;
pub const PRINT_VAL_LABEL: u32 = BEGIN_OF_BUILTIN_LABELS + 2;
pub const INPUT_VAL_LABEL: u32 = BEGIN_OF_BUILTIN_LABELS + 3;
pub const EXIT_LABEL: u32 = BEGIN_OF_BUILTIN_LABELS + 4;

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
            BinaryOperator::And => write!(f, "&&"),
            BinaryOperator::Or => write!(f, "||"),
            // Comparison
            BinaryOperator::Eq => write!(f, "=="),
            BinaryOperator::Ne => write!(f, "!="),
            BinaryOperator::Lt => write!(f, "<"),
            BinaryOperator::Le => write!(f, "<="),
            BinaryOperator::Gt => write!(f, ">"),
            BinaryOperator::Ge => write!(f, ">="),
        }
    }
}

impl From<ast::BinaryOperator> for BinaryOperator {
    fn from(op: ast::BinaryOperator) -> Self {
        match op {
            ast::BinaryOperator::Add => BinaryOperator::Add,
            ast::BinaryOperator::Sub => BinaryOperator::Sub,
            ast::BinaryOperator::Mul => BinaryOperator::Mul,
            ast::BinaryOperator::Div => BinaryOperator::Div,
            ast::BinaryOperator::And => BinaryOperator::And,
            ast::BinaryOperator::Or => BinaryOperator::Or,
            ast::BinaryOperator::Eq => BinaryOperator::Eq,
            ast::BinaryOperator::Ne => BinaryOperator::Ne,
            ast::BinaryOperator::Lt => BinaryOperator::Lt,
            ast::BinaryOperator::Le => BinaryOperator::Le,
            ast::BinaryOperator::Gt => BinaryOperator::Gt,
            ast::BinaryOperator::Ge => BinaryOperator::Ge,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Operand {
    TempVariable { id: u32 },
    // IndirectTemp { id: u32 },
    Variable { id: u32 },
    IndirectVariable { id: u32 },

    NumberLiteral { value: i32 },
    IndirectNumberLiteral { value: i32 },
}

impl Operand {
    pub fn variable_id(&self) -> Option<u32> {
        match self {
            Operand::Variable { id } => Some(*id),
            _ => None,
        }
    }

    pub fn number_literal_value(&self) -> Option<i32> {
        match self {
            Operand::NumberLiteral { value } => Some(*value),
            _ => None,
        }
    }
}

impl std::fmt::Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::TempVariable { id } => write!(f, "t{}", id),
            Operand::Variable { id } => write!(f, "v{}", id),
            Operand::NumberLiteral { value } => write!(f, "{}", value),
            Operand::IndirectVariable { id } => write!(f, "*v{}", id),
            Operand::IndirectNumberLiteral { value } => write!(f, "*{}", value),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Tac {
    // Expressions
    BinExpression {
        left: Operand,
        op: BinaryOperator,
        right: Operand,
        dest: Operand,
    },
    // Copy
    Copy {
        src: Operand,
        dest: Operand,
    },
    // Control flow
    Goto {
        label: u32,
    },
    Label {
        id: u32,
    },
    Return,
    If {
        op: BinaryOperator,
        left: Operand,
        right: Operand,
        label: u32,
    },
    // Labels 0-100 are reserved for built-in functions
    ExternCall {
        label: u32,
    },
    Call {
        label: u32,
    },
    Param {
        operand: Operand,
    },
}

impl std::fmt::Display for Tac {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn print_label(f: &mut std::fmt::Formatter<'_>, id: u32) -> std::fmt::Result {
            match id {
                START_LABEL => write!(f, "start"),
                PRINT_PTR_LABEL => write!(f, "print_ptr"),
                INPUT_PTR_LABEL => write!(f, "input_ptr"),
                PRINT_VAL_LABEL => write!(f, "print_val"),
                INPUT_VAL_LABEL => write!(f, "input_val"),
                EXIT_LABEL => write!(f, "exit"),
                _ => write!(f, "l{}", id),
            }
        }

        match self {
            Tac::Copy { src, dest } => write!(f, "{} := {}", dest, src),
            Tac::BinExpression {
                left,
                op,
                right,
                dest,
            } => {
                write!(f, "{} := {} {} {}", dest, left, op, right)
            }
            Tac::Goto { label } => {
                write!(f, "goto ")?;
                print_label(f, *label)
            }
            Tac::Label { id } => print_label(f, *id),
            Tac::Return => write!(f, "return"),
            Tac::If {
                op,
                left,
                right,
                label,
            } => {
                write!(f, "if {} {} {} goto ", left, op, right)?;
                print_label(f, *label)
            }
            Tac::ExternCall { label } => {
                write!(f, "extern_call ")?;
                print_label(f, *label)
            }
            Tac::Call { label } => {
                write!(f, "call ")?;
                print_label(f, *label)
            }
            Tac::Param { operand } => write!(f, "param {}", operand),
        }
    }
}

pub struct Program {
    hir: Vec<Tac>,
}

impl Program {
    pub fn new() -> Self {
        Program { hir: Vec::new() }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Tac> {
        self.hir.iter()
    }
}

impl std::fmt::Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for hir in &self.hir {
            match hir {
                Tac::Label { .. } => {
                    writeln!(f, "{}:", hir)?;
                }
                _ => {
                    writeln!(f, "\t{}", hir)?;
                }
            }
        }
        Ok(())
    }
}

pub trait ProgramVisitor {
    fn visit_program(&mut self, program: &mut Program);
}

impl Program {
    pub fn accept<V: ProgramVisitor>(&mut self, visitor: &mut V) {
        visitor.visit_program(self);
    }
}

pub trait TacVisitor {
    fn visit_binary_expression(
        &mut self,
        left: &Operand,
        op: BinaryOperator,
        right: &Operand,
        dest: &Operand,
    );
    fn visit_copy(&mut self, src: &Operand, dest: &Operand);
    fn visit_goto(&mut self, label: u32);
    fn visit_label(&mut self, id: u32);
    fn visit_return(&mut self);
    fn visit_if(&mut self, op: BinaryOperator, left: &Operand, right: &Operand, label: u32);
    fn visit_call(&mut self, label: u32);
    fn visit_extern_call(&mut self, label: u32);
    fn visit_param(&mut self, operand: &Operand);
}

impl Tac {
    pub fn accept<V: TacVisitor>(&self, visitor: &mut V) {
        match self {
            Tac::BinExpression {
                left,
                op,
                right,
                dest,
            } => visitor.visit_binary_expression(left, *op, right, dest),
            Tac::Copy { src, dest } => visitor.visit_copy(src, dest),
            Tac::Goto { label } => visitor.visit_goto(*label),
            Tac::Label { id } => visitor.visit_label(*id),
            Tac::Return => visitor.visit_return(),
            Tac::If {
                op,
                left,
                right,
                label,
            } => visitor.visit_if(*op, left, right, *label),
            Tac::Call { label } => visitor.visit_call(*label),
            Tac::ExternCall { label } => visitor.visit_extern_call(*label),
            Tac::Param { operand } => visitor.visit_param(operand),
        }
    }
}

pub trait OperandVisitor {
    fn visit_temp_variable(&mut self, id: u32);
    fn visit_variable(&mut self, id: u32);
    fn visit_indirect_variable(&mut self, id: u32);
    fn visit_number_literal(&mut self, value: i32);
    fn visit_indirect_number_literal(&mut self, value: i32);
}

impl Operand {
    pub fn accept<V: OperandVisitor>(&self, visitor: &mut V) {
        match self {
            Operand::TempVariable { id } => visitor.visit_temp_variable(*id),
            Operand::Variable { id } => visitor.visit_variable(*id),
            Operand::IndirectVariable { id } => visitor.visit_indirect_variable(*id),
            Operand::NumberLiteral { value } => visitor.visit_number_literal(*value),
            Operand::IndirectNumberLiteral { value } => {
                visitor.visit_indirect_number_literal(*value)
            }
        }
    }
}
