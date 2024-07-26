use std::{collections::HashMap, mem};

use petgraph::graph::NodeIndex;

use crate::tac::{Operand, Program, ProgramVisitor, Tac, TacVisitor, START_LABEL};

#[derive(Debug)]
pub struct BasicBlock {
    pub start_label: u32,
    pub tacs: Vec<Tac>,
}

impl BasicBlock {
    pub fn new(start_label: u32) -> Self {
        BasicBlock {
            start_label,
            tacs: Vec::new(),
        }
    }

    pub fn push(&mut self, tac: Tac) {
        self.tacs.push(tac);
    }
}

impl std::fmt::Display for BasicBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "BB{}:", self.start_label)?;
        for tac in &self.tacs {
            writeln!(f, "\t{}", tac)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Cfg {
    graph: petgraph::graph::DiGraph<BasicBlock, ()>,
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
        graph[current_block].start_label = START_LABEL;

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
        let current_block = self.graph.node_weight_mut(self.current_block).unwrap();
        current_block.push(Tac::BinExpression {
            left: *left,
            op,
            right: *right,
            dest: *dest,
        });
    }

    fn visit_copy(&mut self, src: &Operand, dest: &Operand) {
        let current_block = self.graph.node_weight_mut(self.current_block).unwrap();
        current_block.push(Tac::Copy {
            src: *src,
            dest: *dest,
        });
    }

    fn visit_goto(&mut self, label: u32) {
        let current_block = self.graph.node_weight_mut(self.current_block).unwrap();
        current_block.push(Tac::Goto { label });

        self.new_block();

        self.branch_stack.push((self.current_block, label));
    }

    fn visit_label(&mut self, id: u32) {
        self.new_block();
        let current_block = self.graph.node_weight_mut(self.current_block).unwrap();
        current_block.push(Tac::Label { id });
        current_block.start_label = id;
        self.label_to_block.insert(id, self.current_block);
    }

    fn visit_return(&mut self) {
        let current_block = self.graph.node_weight_mut(self.current_block).unwrap();
        current_block.push(Tac::Return);

        self.new_block();
    }

    fn visit_if(
        &mut self,
        op: crate::tac::BinaryOperator,
        left: &Operand,
        right: &Operand,
        label: u32,
    ) {
        let current_block = self.graph.node_weight_mut(self.current_block).unwrap();
        current_block.push(Tac::If {
            op,
            left: *left,
            right: *right,
            label,
        });

        let next_block = self.new_block();

        self.graph.add_edge(self.current_block, next_block, ());

        self.branch_stack.push((self.current_block, label));
    }

    fn visit_call(&mut self, label: u32) {
        let current_block = self.graph.node_weight_mut(self.current_block).unwrap();
        current_block.push(Tac::Call { label });

        self.branch_stack.push((self.current_block, label));

        self.new_block();
    }

    fn visit_param(&mut self, operand: &Operand) {
        let current_block = self.graph.node_weight_mut(self.current_block).unwrap();
        current_block.push(Tac::Param { operand: *operand });
    }
}
