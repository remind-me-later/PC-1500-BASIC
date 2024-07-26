use crate::tac::{Program, ProgramVisitor, Tac, TacVisitor};

pub struct BasicBlock {
    pub id: u32,
    pub tacs: Vec<Tac>,
}

impl std::fmt::Display for BasicBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "BB{}:", self.id)?;
        for tac in &self.tacs {
            writeln!(f, "\t{}", tac)?;
        }
        Ok(())
    }
}

pub struct CFG<'a> {
    pub block: &'a BasicBlock,
    pub next: Option<&'a CFG<'a>>,
}

pub struct CFGBuilder<'a> {
    bump: &'a bumpalo::Bump,
    next_id: u32,
    current_block: Option<&'a mut BasicBlock>,
}

impl<'a> ProgramVisitor for CFGBuilder<'a> {
    fn visit_program(&mut self, program: &mut Program) {
        for tac in program.iter() {
            tac.accept(self);
        }
    }
}

impl<'a> TacVisitor for CFGBuilder<'a> {
    fn visit_binary_expression(
        &mut self,
        left: &crate::tac::Operand,
        op: crate::tac::BinaryOperator,
        right: &crate::tac::Operand,
        dest: &crate::tac::Operand,
    ) {
        todo!()
    }

    fn visit_copy(&mut self, src: &crate::tac::Operand, dest: &crate::tac::Operand) {
        todo!()
    }

    fn visit_goto(&mut self, label: u32) {
        todo!()
    }

    fn visit_label(&mut self, id: u32) {
        todo!()
    }

    fn visit_return(&mut self) {
        todo!()
    }

    fn visit_if(
        &mut self,
        op: crate::tac::BinaryOperator,
        left: &crate::tac::Operand,
        right: &crate::tac::Operand,
        label: u32,
    ) {
        todo!()
    }

    fn visit_call(&mut self, label: u32) {
        todo!()
    }

    fn visit_param(&mut self, operand: &crate::tac::Operand) {
        todo!()
    }
}
