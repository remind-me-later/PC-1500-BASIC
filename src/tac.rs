use std::collections::HashMap;

use typed_arena::Arena;

use crate::dag::{Expression, ExpressionVisitor, Program};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TACLeaf {
    NumberLiteral { value: i32 },
    Variable { id: usize },
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Tac<'a> {
    // Arithmetic
    Add {
        left: &'a TACLeaf,
        right: &'a TACLeaf,
        dest: &'a TACLeaf,
    },
    Sub {
        left: &'a TACLeaf,
        right: &'a TACLeaf,
        dest: &'a TACLeaf,
    },
    Mul {
        left: &'a TACLeaf,
        right: &'a TACLeaf,
        dest: &'a TACLeaf,
    },
    Div {
        left: &'a TACLeaf,
        right: &'a TACLeaf,
        dest: &'a TACLeaf,
    },
    // Comparison
    Eq {
        left: &'a TACLeaf,
        right: &'a TACLeaf,
        dest: &'a TACLeaf,
    },
    Ne {
        left: &'a TACLeaf,
        right: &'a TACLeaf,
        dest: &'a TACLeaf,
    },
    Lt {
        left: &'a TACLeaf,
        right: &'a TACLeaf,
        dest: &'a TACLeaf,
    },
    Le {
        left: &'a TACLeaf,
        right: &'a TACLeaf,
        dest: &'a TACLeaf,
    },
    Gt {
        left: &'a TACLeaf,
        right: &'a TACLeaf,
        dest: &'a TACLeaf,
    },
    Ge {
        left: &'a TACLeaf,
        right: &'a TACLeaf,
        dest: &'a TACLeaf,
    },
    // Logical
    And {
        left: &'a TACLeaf,
        right: &'a TACLeaf,
        dest: &'a TACLeaf,
    },
    Or {
        left: &'a TACLeaf,
        right: &'a TACLeaf,
        dest: &'a TACLeaf,
    },

    // Assignment
    Copy {
        src: &'a TACLeaf,
        dest: &'a TACLeaf,
    },
    // Control flow
    Goto {
        label: &'a Tac<'a>,
    },
    GoSub {
        label: &'a Tac<'a>,
    },
    Return,
    IfGoto {
        cond: &'a TACLeaf,
        label: &'a Tac<'a>,
    },
}

pub struct TACBuilder<'a> {
    next_id: usize,
    instructions: Vec<Tac<'a>>,
    program: &'a Program<'a>,

    tac_arena: &'a Arena<Tac<'a>>,
    tac_leaf_arena: &'a Arena<TACLeaf>,

    var_expr_map: HashMap<&'a Expression<'a>, &'a TACLeaf>,
    str_id_map: HashMap<&'a str, &'a TACLeaf>,
}

impl<'a> TACBuilder<'a> {
    pub fn new(
        program: &'a Program<'a>,
        tac_arena: &'a Arena<Tac<'a>>,
        tac_leaf_arena: &'a Arena<TACLeaf>,
    ) -> Self {
        TACBuilder {
            next_id: 0,
            instructions: Vec::new(),
            program,
            var_expr_map: HashMap::new(),
            str_id_map: HashMap::new(),
            tac_arena,
            tac_leaf_arena,
        }
    }
}

impl<'a> ExpressionVisitor<'a, &'a TACLeaf> for TACBuilder<'a> {
    fn visit_number_literal(&mut self, num: i32) -> &'a TACLeaf {
        let res = TACLeaf::NumberLiteral { value: num };
        self.tac_leaf_arena.alloc(res)
    }

    fn visit_variable(&mut self, variable: &'a str) -> &'a TACLeaf {
        self.str_id_map.entry(variable).or_insert_with(|| {
            let id = self.next_id;
            self.next_id += 1;
            let leaf = TACLeaf::Variable { id };
            self.tac_leaf_arena.alloc(leaf)
        })
    }

    fn visit_binary_op(
        &mut self,
        left: &'a crate::dag::Expression<'a>,
        op: crate::dag::BinaryOperator,
        right: &'a crate::dag::Expression<'a>,
    ) -> &'a TACLeaf {
        if !self.var_expr_map.contains_key(left) {
            let leaf = self.tac_leaf_arena.alloc(*left.accept(self));
            self.var_expr_map.insert(left, leaf);
        }

        let left = *self.var_expr_map.get(left).unwrap();

        if !self.var_expr_map.contains_key(right) {
            let leaf = self.tac_leaf_arena.alloc(*right.accept(self));
            self.var_expr_map.insert(right, leaf);
        }

        let right = *self.var_expr_map.get(right).unwrap();

        let dest = TACLeaf::Variable { id: self.next_id };
        self.next_id += 1;
        let dest = self.tac_leaf_arena.alloc(dest);

        let instr = match op {
            crate::dag::BinaryOperator::Add => Tac::Add { left, right, dest },
            crate::dag::BinaryOperator::Sub => Tac::Sub { left, right, dest },
            crate::dag::BinaryOperator::Mul => Tac::Mul { left, right, dest },
            crate::dag::BinaryOperator::Div => Tac::Div { left, right, dest },

            crate::dag::BinaryOperator::Eq => Tac::Eq { left, right, dest },
            crate::dag::BinaryOperator::Ne => Tac::Ne { left, right, dest },
            crate::dag::BinaryOperator::Lt => Tac::Lt { left, right, dest },
            crate::dag::BinaryOperator::Le => Tac::Le { left, right, dest },
            crate::dag::BinaryOperator::Gt => Tac::Gt { left, right, dest },
            crate::dag::BinaryOperator::Ge => Tac::Ge { left, right, dest },

            crate::dag::BinaryOperator::And => Tac::And { left, right, dest },
            crate::dag::BinaryOperator::Or => Tac::Or { left, right, dest },
        };

        self.instructions.push(instr);

        dest
    }
}
