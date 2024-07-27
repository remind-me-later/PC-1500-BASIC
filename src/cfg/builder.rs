use std::{
    borrow::Borrow,
    cell::RefCell,
    collections::HashMap,
    mem,
    rc::{Rc, Weak},
    vec,
};

use crate::tac::{Operand, Program, ProgramVisitor, Tac, TacVisitor};

use super::{BasicBlock, Cfg};

pub struct Builder {
    program: Program,
    current_block: Weak<RefCell<BasicBlock>>,
    head: Weak<RefCell<BasicBlock>>,
    label_to_block: HashMap<u32, Weak<RefCell<BasicBlock>>>,
    branch_stack: Vec<(Weak<RefCell<BasicBlock>>, u32)>,
    arena: Vec<Rc<RefCell<BasicBlock>>>,
}

impl Builder {
    pub fn new(program: Program) -> Self {
        let strong_head = Rc::new(RefCell::new(BasicBlock::new(0)));
        let weak_head = Rc::downgrade(&strong_head);
        let arena = vec![strong_head];

        Builder {
            program,
            current_block: Weak::clone(&weak_head),
            head: weak_head,
            label_to_block: HashMap::new(),
            branch_stack: Vec::new(),
            arena,
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

    fn new_block(&mut self) {
        {
            let upgraded = self.current_block.upgrade().unwrap();
            let current_block = <Rc<_> as Borrow<RefCell<_>>>::borrow(&upgraded).borrow();

            if current_block.tacs.is_empty() {
                return;
            }
        }

        let next_id = self.arena.len() as u32;
        let block = BasicBlock::new(next_id);
        let strong_block = Rc::new(RefCell::new(block));
        let weak_block = Rc::downgrade(&strong_block);

        self.arena.push(strong_block);

        self.current_block = weak_block;
    }
}

impl ProgramVisitor for Builder {
    fn visit_program(&mut self, program: &mut Program) {
        for tac in program.iter() {
            tac.accept(self);
        }

        for (branch, label) in self.branch_stack.iter() {
            let block = self.label_to_block.get(label).unwrap_or_else(|| {
                unreachable!(
                    "Label {} not found, all labels should be correctly set in the TAC",
                    label
                )
            });

            let branch = branch.upgrade().unwrap();
            let mut branch = branch.borrow_mut();

            branch.branch_to = Some(Weak::clone(block));
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
        let upgraded = self.current_block.upgrade().unwrap();
        let mut block = upgraded.borrow_mut();
        block.push(Tac::BinExpression {
            left: *left,
            op,
            right: *right,
            dest: *dest,
        });
    }

    fn visit_copy(&mut self, src: &Operand, dest: &Operand) {
        let upgraded = self.current_block.upgrade().unwrap();
        let mut block = upgraded.borrow_mut();
        block.push(Tac::Copy {
            src: *src,
            dest: *dest,
        });
    }

    fn visit_goto(&mut self, label: u32) {
        {
            let upgraded = self.current_block.upgrade().unwrap();
            let mut block = upgraded.borrow_mut();

            block.push(Tac::Goto { label });
        }

        self.branch_stack
            .push((Weak::clone(&self.current_block), label));

        self.new_block();
    }

    fn visit_label(&mut self, id: u32) {
        let last_block_weak = Weak::clone(&self.current_block);
        self.new_block();

        {
            let link = {
                let upgraded = self.current_block.upgrade().unwrap();
                let mut current_block = upgraded.borrow_mut();

                current_block.push(Tac::Label { id });

                self.label_to_block
                    .insert(id, Weak::clone(&self.current_block));

                if last_block_weak.ptr_eq(&Weak::clone(&self.current_block)) {
                    return;
                }

                !matches!(
                    current_block.tacs.last().unwrap(),
                    Tac::Goto { .. }
                        | Tac::If { .. }
                        | Tac::Call { .. }
                        | Tac::Return
                        | Tac::ExternCall { .. }
                )
            };

            if link {
                let upgraded = last_block_weak.upgrade().unwrap();
                let mut last_block = upgraded.borrow_mut();

                last_block.next_to = Some(Weak::clone(&self.current_block));
            }
        }
    }

    fn visit_return(&mut self) {
        {
            let upgraded = self.current_block.upgrade().unwrap();
            let mut block = upgraded.borrow_mut();

            block.push(Tac::Return);
        }

        self.new_block();
    }

    fn visit_if(
        &mut self,
        op: crate::tac::BinaryOperator,
        left: &Operand,
        right: &Operand,
        label: u32,
    ) {
        let last_block_weak = Weak::clone(&self.current_block);

        {
            let upgraded = self.current_block.upgrade().unwrap();
            let mut last_block = upgraded.borrow_mut();

            last_block.push(Tac::If {
                op,
                left: *left,
                right: *right,
                label,
            });

            self.branch_stack
                .push((Weak::clone(&self.current_block), label));
        }

        self.new_block();

        {
            let upgraded = last_block_weak.upgrade().unwrap();
            let mut last_block = upgraded.borrow_mut();

            last_block.next_to = Some(Weak::clone(&self.current_block));
        }
    }

    fn visit_call(&mut self, label: u32) {
        {
            let upgraded = self.current_block.upgrade().unwrap();
            let mut block = upgraded.borrow_mut();

            block.push(Tac::Call { label });
        }

        self.branch_stack
            .push((Weak::clone(&self.current_block), label));

        self.new_block();
    }

    fn visit_param(&mut self, operand: &Operand) {
        {
            let upgraded = self.current_block.upgrade().unwrap();
            let mut block = upgraded.borrow_mut();
            block.push(Tac::Param { operand: *operand });
        }
    }

    fn visit_extern_call(&mut self, label: u32) {
        let last_block_weak = Weak::clone(&self.current_block);

        {
            let upgraded = self.current_block.upgrade().unwrap();
            let mut last_block = upgraded.borrow_mut();

            last_block.push(Tac::ExternCall { label });
        }

        self.new_block();

        {
            let upgraded = last_block_weak.upgrade().unwrap();
            let mut last_block = upgraded.borrow_mut();
            last_block.next_to = Some(Weak::clone(&self.current_block));
        }
    }
}
