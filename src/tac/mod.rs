mod builder;
mod visitor;

pub use builder::Builder;
pub use visitor::{ProgramVisitor, TacVisitor};

use crate::ast;

type LabelId = u32;
type VariableId = u32;
type NumLiteral = i32;

pub const START_LABEL: LabelId = 0;
pub const BEGIN_OF_BUILTIN_LABELS: LabelId = 1;
pub const END_OF_BUILTIN_LABELS: LabelId = 20;
pub const PRINT_PTR_LABEL: LabelId = BEGIN_OF_BUILTIN_LABELS;
pub const INPUT_PTR_LABEL: LabelId = BEGIN_OF_BUILTIN_LABELS + 1;
pub const PRINT_VAL_LABEL: LabelId = BEGIN_OF_BUILTIN_LABELS + 2;
pub const INPUT_VAL_LABEL: LabelId = BEGIN_OF_BUILTIN_LABELS + 3;
pub const EXIT_LABEL: LabelId = BEGIN_OF_BUILTIN_LABELS + 4;

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
    Cp, // Compare
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
            BinaryOperator::Cp => write!(f, "=="),
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
            ast::BinaryOperator::Eq
            | ast::BinaryOperator::Ne
            | ast::BinaryOperator::Lt
            | ast::BinaryOperator::Le
            | ast::BinaryOperator::Gt
            | ast::BinaryOperator::Ge => {
                panic!("Invalid conversion from ast::BinaryOperator to tac::BinaryOperator")
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComparisonOperator {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

impl ComparisonOperator {
    pub fn negate(&self) -> Self {
        match self {
            ComparisonOperator::Eq => ComparisonOperator::Ne,
            ComparisonOperator::Ne => ComparisonOperator::Eq,
            ComparisonOperator::Lt => ComparisonOperator::Ge,
            ComparisonOperator::Le => ComparisonOperator::Gt,
            ComparisonOperator::Gt => ComparisonOperator::Le,
            ComparisonOperator::Ge => ComparisonOperator::Lt,
        }
    }
}

impl std::fmt::Display for ComparisonOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComparisonOperator::Eq => write!(f, "=="),
            ComparisonOperator::Ne => write!(f, "!="),
            ComparisonOperator::Lt => write!(f, "<"),
            ComparisonOperator::Le => write!(f, "<="),
            ComparisonOperator::Gt => write!(f, ">"),
            ComparisonOperator::Ge => write!(f, ">="),
        }
    }
}

impl From<ast::BinaryOperator> for ComparisonOperator {
    fn from(op: ast::BinaryOperator) -> Self {
        match op {
            ast::BinaryOperator::Eq => ComparisonOperator::Eq,
            ast::BinaryOperator::Ne => ComparisonOperator::Ne,
            ast::BinaryOperator::Lt => ComparisonOperator::Lt,
            ast::BinaryOperator::Le => ComparisonOperator::Le,
            ast::BinaryOperator::Gt => ComparisonOperator::Gt,
            ast::BinaryOperator::Ge => ComparisonOperator::Ge,
            ast::BinaryOperator::Add
            | ast::BinaryOperator::Sub
            | ast::BinaryOperator::Mul
            | ast::BinaryOperator::Div
            | ast::BinaryOperator::And
            | ast::BinaryOperator::Or => {
                panic!("Invalid conversion from ast::BinaryOperator to tac::ComparisonOperator")
            }
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Operand {
    Variable { id: VariableId },
    IndirectVariable { id: VariableId },

    NumberLiteral { value: NumLiteral },
    IndirectNumberLiteral { value: NumLiteral },
}

impl Operand {
    pub fn variable_id(&self) -> Option<VariableId> {
        match self {
            Operand::Variable { id } => Some(*id),
            _ => None,
        }
    }

    pub fn number_literal_value(&self) -> Option<NumLiteral> {
        match self {
            Operand::NumberLiteral { value } => Some(*value),
            _ => None,
        }
    }
}

impl std::fmt::Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
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
        label: LabelId,
    },
    Label {
        label: LabelId,
    },
    Return,
    If {
        op: ComparisonOperator,
        left: Operand,
        right: Operand,
        label: LabelId,
    },
    // Labels 0-100 are reserved for built-in functions
    ExternCall {
        label: LabelId,
    },
    Call {
        label: LabelId,
    },
    Param {
        operand: Operand,
    },
}

impl std::fmt::Display for Tac {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn print_label(f: &mut std::fmt::Formatter<'_>, id: LabelId) -> std::fmt::Result {
            match id {
                START_LABEL => write!(f, "start"),
                PRINT_PTR_LABEL => write!(f, "print_str"),
                INPUT_PTR_LABEL => write!(f, "input_str"),
                PRINT_VAL_LABEL => write!(f, "print"),
                INPUT_VAL_LABEL => write!(f, "input"),
                EXIT_LABEL => write!(f, "exit"),
                _ => write!(f, "l{}", id),
            }
        }

        match self {
            Tac::Copy { src, dest } => write!(f, "{} = {}", dest, src),
            Tac::BinExpression {
                left,
                op,
                right,
                dest,
            } => {
                write!(f, "{} = {} {} {}", dest, left, op, right)
            }
            Tac::Goto { label } => {
                write!(f, "goto ")?;
                print_label(f, *label)
            }
            Tac::Label { label: id } => print_label(f, *id),
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
                write!(f, "call? ")?;
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
