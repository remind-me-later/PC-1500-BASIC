mod tac_builder;

pub use tac_builder::HirBuilder;

pub const BEGIN_OF_BUILTIN_LABELS: u32 = 0;
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

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Operand {
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
            Operand::Variable { id } => write!(f, "v{}", id),
            Operand::NumberLiteral { value } => write!(f, "{}", value),
            Operand::IndirectVariable { id } => write!(f, "*v{}", id),
            Operand::IndirectNumberLiteral { value } => write!(f, "*{}", value),
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct Expression {
    left: Operand,
    op: BinaryOperator,
    right: Operand,
    dest: Operand,
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} := {} {} {}",
            self.dest, self.left, self.op, self.right
        )
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct IfCondition {
    left: Operand,
    op: BinaryOperator,
    right: Operand,
}

impl std::fmt::Display for IfCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.left, self.op, self.right)
    }
}

pub enum Tac {
    Expression(Expression),
    Copy { src: Operand, dest: Operand },
    // Control flow
    Goto { label: u32 },
    Label { id: u32 },
    Return,
    If { condition: IfCondition, label: u32 },
    // Labels 0-100 are reserved for built-in functions
    Call { label: u32 },
    Param { operand: Operand },
}

impl std::fmt::Display for Tac {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tac::Copy { src, dest } => write!(f, "{} := {}", dest, src),
            Tac::Expression(expr) => write!(f, "{}", expr),
            Tac::Goto { label } => write!(f, "goto l{}", label),
            Tac::Label { id } => write!(f, "l{}", id),
            Tac::Return => write!(f, "return"),
            Tac::If { condition, label } => write!(f, "if {} goto l{}", condition, label),
            Tac::Call { label } => match *label {
                PRINT_PTR_LABEL => write!(f, "call print_ptr"),
                INPUT_PTR_LABEL => write!(f, "call input_ptr"),
                PRINT_VAL_LABEL => write!(f, "call print_val"),
                INPUT_VAL_LABEL => write!(f, "call input_val"),
                EXIT_LABEL => write!(f, "call exit"),
                _ => write!(f, "call l{}", label),
            },
            Tac::Param { operand } => write!(f, "param {}", operand),
        }
    }
}

pub struct Program {
    hir: Vec<Tac>,
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
    fn visit_expression(&mut self, expr: &mut Expression);
    fn visit_copy(&mut self, src: &mut Operand, dest: &mut Operand);
    fn visit_goto(&mut self, label: u32);
    fn visit_label(&mut self, id: u32);
    fn visit_return(&mut self);
    fn visit_if(&mut self, condition: &mut IfCondition, label: u32);
    fn visit_call(&mut self, label: u32);
    fn visit_param(&mut self, operand: &mut Operand);
}

impl Tac {
    pub fn accept<V: TacVisitor>(&mut self, visitor: &mut V) {
        match self {
            Tac::Expression(expr) => visitor.visit_expression(expr),
            Tac::Copy { src, dest } => visitor.visit_copy(src, dest),
            Tac::Goto { label } => visitor.visit_goto(*label),
            Tac::Label { id } => visitor.visit_label(*id),
            Tac::Return => visitor.visit_return(),
            Tac::If { condition, label } => visitor.visit_if(condition, *label),
            Tac::Call { label } => visitor.visit_call(*label),
            Tac::Param { operand } => visitor.visit_param(operand),
        }
    }
}

pub trait OperandVisitor {
    fn visit_variable(&mut self, id: u32);
    fn visit_indirect_variable(&mut self, id: u32);
    fn visit_number_literal(&mut self, value: i32);
    fn visit_indirect_number_literal(&mut self, value: i32);
}

impl Operand {
    pub fn accept<V: OperandVisitor>(&self, visitor: &mut V) {
        match self {
            Operand::Variable { id } => visitor.visit_variable(*id),
            Operand::IndirectVariable { id } => visitor.visit_indirect_variable(*id),
            Operand::NumberLiteral { value } => visitor.visit_number_literal(*value),
            Operand::IndirectNumberLiteral { value } => {
                visitor.visit_indirect_number_literal(*value)
            }
        }
    }
}
