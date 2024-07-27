use std::{collections::HashMap, mem};

use crate::tac::{Operand, Program, ProgramVisitor, Tac, TacVisitor};

use super::{BasicBlock, Cfg};

pub struct Builder {
    program: Program,
    next_id: u32,
    current_block: *mut BasicBlock,
    head: *mut BasicBlock,
    label_to_block: HashMap<u32, *mut BasicBlock>,
    branch_stack: Vec<(*mut BasicBlock, u32)>,
    arena: Vec<BasicBlock>,
}

impl Builder {
    pub fn new(program: Program) -> Self {
        let head = Box::into_raw(Box::new(BasicBlock::new(0)));
        let current_block = head;

        Builder {
            program,
            next_id: 1,
            current_block,
            head,
            label_to_block: HashMap::new(),
            branch_stack: Vec::new(),
            arena: Vec::new(),
        }
    }

    pub fn build(mut self) -> Cfg {
        let mut program = mem::replace(&mut self.program, Program::new());
        program.accept(&mut self);

        Cfg {
            head: self.head,
            arena: self.arena,
        }
    }

    fn new_block(&mut self) -> *mut BasicBlock {
        let current_block = unsafe { &mut *self.current_block };
        if current_block.tacs.is_empty() {
            return self.current_block;
        }

        let block = BasicBlock::new(self.next_id);
        let idx = self.arena.len();
        self.arena.push(block);
        self.current_block = &mut self.arena[idx] as *mut BasicBlock;
        self.next_id += 1;
        self.current_block
    }

    fn current_block_mut(&mut self) -> &mut BasicBlock {
        unsafe { &mut *self.current_block }
    }
}

impl ProgramVisitor for Builder {
    fn visit_program(&mut self, program: &mut Program) {
        for tac in program.iter() {
            tac.accept(self);
        }

        for (branch, label) in self.branch_stack.iter() {
            let block = self.label_to_block.get(label).unwrap();
            let branch = unsafe { &mut **branch };
            branch.next_branch = Some(*block);
        }
    }
}

impl TacVisitor for Builder {
    fn visit_binary_expression(
        &mut self,
        left: &Operand,
        op: crate::tac::BinaryOperator,
        right: &Operand,
        dest: &Operand,
    ) {
        self.current_block_mut().push(Tac::BinExpression {
            left: *left,
            op,
            right: *right,
            dest: *dest,
        });
    }

    fn visit_copy(&mut self, src: &Operand, dest: &Operand) {
        self.current_block_mut().push(Tac::Copy {
            src: *src,
            dest: *dest,
        });
    }

    fn visit_goto(&mut self, label: u32) {
        self.current_block_mut().push(Tac::Goto { label });

        self.branch_stack.push((self.current_block, label));

        self.new_block();
    }

    fn visit_label(&mut self, id: u32) {
        let last_block_idx = self.current_block;
        self.new_block();
        let current_block_idx = self.current_block;

        self.current_block_mut().push(Tac::Label { id });

        self.label_to_block.insert(id, current_block_idx);

        if last_block_idx == current_block_idx {
            return;
        }

        let last_block = unsafe { &mut *last_block_idx };

        match last_block.tacs.last().unwrap() {
            Tac::Goto { .. } | Tac::If { .. } | Tac::Call { .. } | Tac::Return => {}
            _ => {
                last_block.next_linear = Some(current_block_idx);
            }
        }
    }

    fn visit_return(&mut self) {
        self.current_block_mut().push(Tac::Return);

        self.new_block();
    }

    fn visit_if(
        &mut self,
        op: crate::tac::BinaryOperator,
        left: &Operand,
        right: &Operand,
        label: u32,
    ) {
        let current_block_ptr = self.current_block;
        self.current_block_mut().push(Tac::If {
            op,
            left: *left,
            right: *right,
            label,
        });

        self.branch_stack.push((self.current_block, label));

        let new_block_ptr = self.new_block();

        let current_block = unsafe { &mut *current_block_ptr };
        current_block.next_linear = Some(new_block_ptr);
    }

    fn visit_call(&mut self, label: u32) {
        self.current_block_mut().push(Tac::Call { label });

        self.branch_stack.push((self.current_block, label));

        self.new_block();
    }

    fn visit_param(&mut self, operand: &Operand) {
        self.current_block_mut()
            .push(Tac::Param { operand: *operand });
    }
}
