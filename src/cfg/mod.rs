mod builder;

pub use builder::Builder;

use std::{
    borrow::Borrow,
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

use crate::tac::{Operand, Tac};

#[derive(Debug, Clone)]
pub struct BasicBlock {
    pub id: u32,
    pub tacs: Vec<Tac>,
    pub next_to: Option<Weak<RefCell<BasicBlock>>>,
    pub branch_to: Option<Weak<RefCell<BasicBlock>>>,
}

impl BasicBlock {
    pub fn new(id: u32) -> Self {
        BasicBlock {
            id,
            tacs: Vec::new(),
            next_to: None,
            branch_to: None,
        }
    }

    pub fn push(&mut self, tac: Tac) {
        self.tacs.push(tac);
    }

    pub fn last(&self) -> Option<&Tac> {
        self.tacs.last()
    }

    pub fn constant_fold(&mut self) {
        let mut var_val: HashMap<Operand, i32> = HashMap::new();

        let mut new_tacs = Vec::new();

        for tac in self.tacs.iter() {
            match tac {
                Tac::BinExpression {
                    left,
                    op,
                    right,
                    dest,
                } => {
                    let left_val = var_val.get(left).copied().or({
                        if let Operand::NumberLiteral { value } = left {
                            Some(*value)
                        } else {
                            None
                        }
                    });
                    let right_val = var_val.get(right).copied().or({
                        if let Operand::NumberLiteral { value } = right {
                            Some(*value)
                        } else {
                            None
                        }
                    });

                    if let (Some(left_val), Some(right_val)) = (left_val, right_val) {
                        // TODO: are the semantics equal to the original?
                        let result = match op {
                            crate::tac::BinaryOperator::Add => left_val + right_val,
                            crate::tac::BinaryOperator::Sub => left_val - right_val,
                            crate::tac::BinaryOperator::Mul => left_val * right_val,
                            crate::tac::BinaryOperator::Div => left_val / right_val,

                            crate::tac::BinaryOperator::And => {
                                (left_val != 0 && right_val != 0) as i32
                            }
                            crate::tac::BinaryOperator::Or => {
                                (left_val != 0 || right_val != 0) as i32
                            }
                        };

                        var_val.insert(*dest, result);

                        // TODO: necessary?
                        new_tacs.push(Tac::Copy {
                            src: Operand::NumberLiteral { value: result },
                            dest: *dest,
                        });
                    } else {
                        new_tacs.push(Tac::BinExpression {
                            left: *left,
                            op: *op,
                            right: *right,
                            dest: *dest,
                        });
                    }
                }
                Tac::Copy { src, dest } => {
                    if let Some(&val) = var_val.get(src) {
                        var_val.insert(*dest, val);
                        new_tacs.push(Tac::Copy {
                            src: Operand::NumberLiteral { value: val },
                            dest: *dest,
                        });
                    } else {
                        match src {
                            Operand::NumberLiteral { value } => {
                                var_val.insert(*dest, *value);
                                new_tacs.push(Tac::Copy {
                                    src: Operand::NumberLiteral { value: *value },
                                    dest: *dest,
                                });
                            }
                            _ => {
                                new_tacs.push(Tac::Copy {
                                    src: *src,
                                    dest: *dest,
                                });
                            }
                        }
                    }
                }

                Tac::Label { label: id } => {
                    new_tacs.push(Tac::Label { label: *id });
                }
                Tac::Param { operand } => {
                    if let Some(val) = var_val.get(operand) {
                        new_tacs.push(Tac::Param {
                            operand: Operand::NumberLiteral { value: *val },
                        });
                    } else {
                        new_tacs.push(Tac::Param { operand: *operand });
                    }
                }

                // Branching, must be last instruction in block, by construction
                Tac::If {
                    op,
                    left,
                    right,
                    label,
                } => {
                    if let (Some(left_val), Some(right_val)) = (
                        var_val.get(left).copied().or({
                            if let Operand::NumberLiteral { value } = left {
                                Some(*value)
                            } else {
                                None
                            }
                        }),
                        var_val.get(right).copied().or({
                            if let Operand::NumberLiteral { value } = right {
                                Some(*value)
                            } else {
                                None
                            }
                        }),
                    ) {
                        let result = match op {
                            crate::tac::ComparisonOperator::Eq => (left_val == right_val) as i32,
                            crate::tac::ComparisonOperator::Ne => (left_val != right_val) as i32,
                            crate::tac::ComparisonOperator::Lt => (left_val < right_val) as i32,
                            crate::tac::ComparisonOperator::Le => (left_val <= right_val) as i32,
                            crate::tac::ComparisonOperator::Gt => (left_val > right_val) as i32,
                            crate::tac::ComparisonOperator::Ge => (left_val >= right_val) as i32,
                        };

                        if result != 0 {
                            new_tacs.push(Tac::Goto { label: *label });
                            self.next_to = None;
                        } else {
                            self.branch_to = None;
                        }
                    } else {
                        new_tacs.push(Tac::If {
                            op: *op,
                            left: *left,
                            right: *right,
                            label: *label,
                        });
                    }
                }

                Tac::Goto { label } => {
                    new_tacs.push(Tac::Goto { label: *label });
                }

                Tac::Return => {
                    new_tacs.push(Tac::Return);
                }

                Tac::Call { label } => {
                    new_tacs.push(Tac::Call { label: *label });
                }

                Tac::ExternCall { label } => {
                    new_tacs.push(Tac::ExternCall { label: *label });
                }
            }
        }

        self.tacs = new_tacs;
    }
}

impl std::fmt::Display for BasicBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut i = 0;

        if self.tacs.is_empty() {
            return Ok(());
        } else if let Tac::Label { label } = self.tacs[0] {
            writeln!(f, "╔═════ {} : l{} ═══", self.id, label)?;
            i += 1;
        } else {
            writeln!(f, "╔═════ {} ═════════", self.id)?;
        }

        for tac in &self.tacs[i..] {
            writeln!(f, "║\t{}", tac)?;
        }

        write!(f, "╚═════ ")?;

        if let (Some(next_to), Some(branch_to)) = (&self.next_to, &self.branch_to) {
            let upgraded = next_to.upgrade().unwrap();
            let block = <Rc<_> as Borrow<RefCell<_>>>::borrow(&upgraded).borrow();
            write!(f, "{} | ", block.id)?;

            let upgraded = branch_to.upgrade().unwrap();
            let block = <Rc<_> as Borrow<RefCell<_>>>::borrow(&upgraded).borrow();
            write!(f, "{} ════", block.id)?;
        } else if let Some(next_to) = &self.next_to {
            let upgraded = next_to.upgrade().unwrap();
            let block = <Rc<_> as Borrow<RefCell<_>>>::borrow(&upgraded).borrow();
            write!(f, "{} ═════════", block.id)?;
        } else if let Some(branch_to) = &self.branch_to {
            let upgraded = branch_to.upgrade().unwrap();
            let block = <Rc<_> as Borrow<RefCell<_>>>::borrow(&upgraded).borrow();
            write!(f, "{} ═════════", block.id)?;
        } else {
            write!(f, "end ════════")?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Cfg {
    arena: Vec<Rc<RefCell<BasicBlock>>>,
    head: Weak<RefCell<BasicBlock>>,
}

impl Cfg {
    pub fn constant_fold(&mut self) {
        for node in self.arena.iter_mut() {
            let mut node = node.borrow_mut();
            node.constant_fold();
        }
    }
}

impl std::fmt::Display for Cfg {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut stack = Vec::new();
        let mut visited = std::collections::HashSet::new();

        stack.push(Weak::clone(&self.head));

        while let Some(node) = stack.pop() {
            let node = node.upgrade().unwrap();
            let node = <Rc<_> as Borrow<RefCell<_>>>::borrow(&node).borrow();

            if !visited.insert(node.id) {
                continue;
            }

            writeln!(f, "{}\n", node)?;

            if let Some(next_to) = &node.next_to {
                stack.push(Weak::clone(next_to));
            }

            if let Some(branch_to) = &node.branch_to {
                stack.push(Weak::clone(branch_to));
            }
        }

        Ok(())
    }
}
