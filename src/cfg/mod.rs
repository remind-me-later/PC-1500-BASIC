use std::{collections::HashMap, mem};

use petgraph::graph::NodeIndex;

use crate::tac::{Operand, Program, ProgramVisitor, Tac, TacVisitor};

#[derive(Debug, Clone, Copy)]
pub enum FoldResult {
    Unchanged,
    Linear,
    Branch,
}

#[derive(Debug)]
pub struct BasicBlock {
    pub id: u32,
    pub tacs: Vec<Tac>,
}

impl BasicBlock {
    pub fn new(id: u32) -> Self {
        BasicBlock {
            id,
            tacs: Vec::new(),
        }
    }

    pub fn push(&mut self, tac: Tac) {
        self.tacs.push(tac);
    }

    pub fn constant_fold(&mut self) -> FoldResult {
        let mut var_val: HashMap<Operand, i32> = HashMap::new();

        let mut new_tacs = Vec::new();
        let mut fold_result = FoldResult::Unchanged;

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

                            crate::tac::BinaryOperator::Eq => (left_val == right_val) as i32,
                            crate::tac::BinaryOperator::Ne => (left_val != right_val) as i32,
                            crate::tac::BinaryOperator::Lt => (left_val < right_val) as i32,
                            crate::tac::BinaryOperator::Le => (left_val <= right_val) as i32,
                            crate::tac::BinaryOperator::Gt => (left_val > right_val) as i32,
                            crate::tac::BinaryOperator::Ge => (left_val >= right_val) as i32,
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

                Tac::Label { id } => {
                    new_tacs.push(Tac::Label { id: *id });
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
                            crate::tac::BinaryOperator::Eq => (left_val == right_val) as i32,
                            crate::tac::BinaryOperator::Ne => (left_val != right_val) as i32,
                            crate::tac::BinaryOperator::Lt => (left_val < right_val) as i32,
                            crate::tac::BinaryOperator::Le => (left_val <= right_val) as i32,
                            crate::tac::BinaryOperator::Gt => (left_val > right_val) as i32,
                            crate::tac::BinaryOperator::Ge => (left_val >= right_val) as i32,
                            _ => unreachable!(),
                        };

                        if result != 0 {
                            new_tacs.push(Tac::Goto { label: *label });
                            fold_result = FoldResult::Branch;
                        } else {
                            fold_result = FoldResult::Linear;
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
            }
        }

        self.tacs = new_tacs;

        fold_result
    }
}

impl std::fmt::Display for BasicBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "=== BB:{} ===", self.id)?;
        for tac in &self.tacs {
            match tac {
                Tac::Label { .. } => {
                    writeln!(f, "{}:", tac)?;
                }
                _ => {
                    writeln!(f, "\t{}", tac)?;
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Cfg {
    graph: petgraph::graph::DiGraph<BasicBlock, ()>,
}

impl Cfg {
    pub fn constant_fold(&mut self) {
        let mut indices = self.graph.node_indices().collect::<Vec<_>>();

        while let Some(node) = indices.pop() {
            let block = self.graph.node_weight_mut(node).unwrap();
            let block_id = block.id;
            let res = block.constant_fold();
            match res {
                FoldResult::Unchanged => {}
                FoldResult::Linear => {
                    let mut next = None;
                    let neighbours = self.graph.neighbors(node).collect::<Vec<_>>();

                    for neighbour in neighbours {
                        let neighbour_block = self.graph.node_weight(neighbour).unwrap();
                        if neighbour_block.id == block_id + 1 {
                            next = Some(neighbour);
                        }
                        let edge = self.graph.find_edge(node, neighbour).unwrap();
                        self.graph.remove_edge(edge);
                    }

                    if let Some(next) = next {
                        let next_neighbours = self.graph.neighbors(next).collect::<Vec<_>>();
                        let mut next_block = self.graph.remove_node(next).unwrap();
                        indices.retain(|&idx| idx != next);

                        let block = self.graph.node_weight_mut(node).unwrap();

                        block.tacs.append(&mut next_block.tacs);
                        for neighbour in next_neighbours {
                            self.graph.add_edge(node, neighbour, ());
                        }
                    } else {
                        unreachable!();
                    }
                }
                FoldResult::Branch => {
                    let mut next = None;
                    let neighbours = self.graph.neighbors(node).collect::<Vec<_>>();

                    for neighbour in neighbours {
                        let neighbour_block = self.graph.node_weight(neighbour).unwrap();
                        if neighbour_block.id != block_id + 1 {
                            next = Some(neighbour);
                        }
                        let edge = self.graph.find_edge(node, neighbour).unwrap();
                        self.graph.remove_edge(edge);
                    }

                    if let Some(next) = next {
                        self.graph.add_edge(node, next, ());
                    } else {
                        unreachable!();
                    }
                }
            }
        }

        // remove empty blocks
        self.graph
            .retain_nodes(|block, idx| !block.node_weight(idx).unwrap().tacs.is_empty());
    }
}

impl std::fmt::Display for Cfg {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for node in self.graph.node_indices() {
            write!(f, "{}", self.graph[node])?;
            let neighbours = self.graph.neighbors(node);
            write!(f, "==> ")?;
            for neighbour in neighbours {
                write!(f, "BB:{} ", self.graph[neighbour].id)?;
            }
            writeln!(f, "<==")?;
            writeln!(f)?;
        }

        Ok(())
    }
}

pub struct CFGBuilder {
    program: Program,
    next_id: u32,
    current_block: NodeIndex,
    graph: petgraph::graph::DiGraph<BasicBlock, ()>,
    label_to_block: HashMap<u32, NodeIndex>,
    branch_stack: Vec<(NodeIndex, u32)>,
}

impl CFGBuilder {
    pub fn new(program: Program) -> Self {
        let mut graph = petgraph::graph::DiGraph::new();
        let current_block = graph.add_node(BasicBlock::new(0));

        CFGBuilder {
            program,
            next_id: 1,
            current_block,
            graph,
            label_to_block: HashMap::new(),
            branch_stack: Vec::new(),
        }
    }

    pub fn build(mut self) -> Cfg {
        let mut program = mem::replace(&mut self.program, Program::new());
        program.accept(&mut self);

        Cfg { graph: self.graph }
    }

    fn new_block(&mut self) -> NodeIndex {
        let current_block = self.graph.node_weight(self.current_block).unwrap();
        if current_block.tacs.is_empty() {
            return self.current_block;
        }

        let block = BasicBlock::new(self.next_id);
        self.current_block = self.graph.add_node(block);
        self.next_id += 1;
        self.current_block
    }
}

impl ProgramVisitor for CFGBuilder {
    fn visit_program(&mut self, program: &mut Program) {
        for tac in program.iter() {
            tac.accept(self);
        }

        for (branch, label) in self.branch_stack.iter() {
            let block = self.label_to_block.get(label).unwrap();
            self.graph.add_edge(*branch, *block, ());
        }
    }
}

impl TacVisitor for CFGBuilder {
    fn visit_binary_expression(
        &mut self,
        left: &Operand,
        op: crate::tac::BinaryOperator,
        right: &Operand,
        dest: &Operand,
    ) {
        self.graph
            .node_weight_mut(self.current_block)
            .unwrap()
            .push(Tac::BinExpression {
                left: *left,
                op,
                right: *right,
                dest: *dest,
            });
    }

    fn visit_copy(&mut self, src: &Operand, dest: &Operand) {
        self.graph
            .node_weight_mut(self.current_block)
            .unwrap()
            .push(Tac::Copy {
                src: *src,
                dest: *dest,
            });
    }

    fn visit_goto(&mut self, label: u32) {
        self.graph
            .node_weight_mut(self.current_block)
            .unwrap()
            .push(Tac::Goto { label });

        self.branch_stack.push((self.current_block, label));

        self.new_block();
    }

    fn visit_label(&mut self, id: u32) {
        let last_block_idx = self.current_block;
        self.new_block();
        let current_block_idx = self.current_block;

        self.graph
            .node_weight_mut(current_block_idx)
            .unwrap()
            .push(Tac::Label { id });

        self.label_to_block.insert(id, current_block_idx);

        if last_block_idx == current_block_idx {
            return;
        }

        let last_block = self.graph.node_weight(last_block_idx).unwrap();

        match last_block.tacs.last().unwrap() {
            Tac::Goto { .. } | Tac::If { .. } | Tac::Call { .. } | Tac::Return => {}
            _ => {
                self.graph.add_edge(last_block_idx, current_block_idx, ());
            }
        }
    }

    fn visit_return(&mut self) {
        self.graph
            .node_weight_mut(self.current_block)
            .unwrap()
            .push(Tac::Return);

        self.new_block();
    }

    fn visit_if(
        &mut self,
        op: crate::tac::BinaryOperator,
        left: &Operand,
        right: &Operand,
        label: u32,
    ) {
        let current_block_idx = self.current_block;
        self.graph
            .node_weight_mut(self.current_block)
            .unwrap()
            .push(Tac::If {
                op,
                left: *left,
                right: *right,
                label,
            });

        self.branch_stack.push((self.current_block, label));

        let new_block_idx = self.new_block();

        self.graph.add_edge(current_block_idx, new_block_idx, ());
    }

    fn visit_call(&mut self, label: u32) {
        self.graph
            .node_weight_mut(self.current_block)
            .unwrap()
            .push(Tac::Call { label });

        self.branch_stack.push((self.current_block, label));

        self.new_block();
    }

    fn visit_param(&mut self, operand: &Operand) {
        self.graph
            .node_weight_mut(self.current_block)
            .unwrap()
            .push(Tac::Param { operand: *operand });
    }
}
