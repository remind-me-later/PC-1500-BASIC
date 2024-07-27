use super::{BinaryOperator, Operand, Program, Tac};

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
