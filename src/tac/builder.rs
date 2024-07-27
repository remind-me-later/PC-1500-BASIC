use std::collections::HashMap;

use crate::{
    ast::{self, ExpressionVisitor},
    tac::{EXIT_LABEL, INPUT_PTR_LABEL, PRINT_PTR_LABEL},
};

use super::{
    BinaryOperator, Operand, Program, Tac, END_OF_BUILTIN_LABELS, INPUT_VAL_LABEL, PRINT_VAL_LABEL,
};

struct ForInfo<'a> {
    begin_label: u32,
    end_label: u32,
    step: Option<&'a ast::Expression<'a>>,
}

pub struct Builder<'a> {
    hir: Vec<Tac>,

    program: &'a ast::Program<'a>,

    var_map: HashMap<&'a str, Operand>,
    expr_map: HashMap<&'a ast::Expression<'a>, Operand>,

    str_map: HashMap<&'a str, usize>,
    str_literals: Vec<String>,

    line_to_hir_map: HashMap<usize, usize>,

    for_stack: Vec<ForInfo<'a>>,
    goto_list: Vec<usize>,

    next_variable: u32,
    next_label: u32,
}

impl<'a> Builder<'a> {
    pub fn new(program: &'a ast::Program<'a>) -> Self {
        Self {
            program,
            hir: Vec::new(),
            var_map: HashMap::new(),
            expr_map: HashMap::new(),
            line_to_hir_map: HashMap::new(),
            for_stack: Vec::new(),
            next_variable: 0,
            next_label: END_OF_BUILTIN_LABELS,
            str_map: HashMap::new(),
            str_literals: Vec::new(),
            goto_list: Vec::new(),
        }
    }

    pub fn build(mut self) -> (Program, Vec<String>) {
        self.program.accept(&mut self);
        let program = Program { hir: self.hir };
        (program, self.str_literals)
    }

    fn get_next_variable_id(&mut self) -> u32 {
        let id = self.next_variable;
        self.next_variable += 1;
        id
    }

    fn get_next_label(&mut self) -> u32 {
        let label = self.next_label;
        self.next_label += 1;
        label
    }

    fn insert_str_literal(&mut self, s: &'a str) -> u32 {
        // TODO: check overflows
        if let Some(&id) = self.str_map.get(s) {
            id as u32
        } else {
            let id = self.str_literals.len();
            self.str_literals.push(s.to_string());
            self.str_map.insert(s, id);
            id as u32
        }
    }

    fn negate_comparison(op: BinaryOperator) -> BinaryOperator {
        match op {
            BinaryOperator::Eq => BinaryOperator::Ne,
            BinaryOperator::Ne => BinaryOperator::Eq,
            BinaryOperator::Lt => BinaryOperator::Ge,
            BinaryOperator::Le => BinaryOperator::Gt,
            BinaryOperator::Gt => BinaryOperator::Le,
            BinaryOperator::Ge => BinaryOperator::Lt,
            _ => unreachable!("Invalid comparison operator: {:?}", op),
        }
    }
}

impl<'a> ast::ExpressionVisitor<'a, Operand> for Builder<'a> {
    fn visit_number_literal(&mut self, value: i32) -> Operand {
        Operand::NumberLiteral { value }
    }

    fn visit_string_literal(&mut self, content: &'a str) -> Operand {
        let index = self.insert_str_literal(content);

        Operand::IndirectNumberLiteral {
            value: index as i32,
        }
    }

    fn visit_variable(&mut self, variable: &'a str) -> Operand {
        if let Some(&id) = self.var_map.get(variable) {
            id
        } else {
            let id = self.get_next_variable_id();

            let var = if variable.trim().ends_with("$") {
                Operand::IndirectVariable { id }
            } else {
                Operand::Variable { id }
            };

            self.var_map.insert(variable, var);

            var
        }
    }

    fn visit_binary_op(
        &mut self,
        left: &'a ast::Expression<'a>,
        op: ast::BinaryOperator,
        right: &'a ast::Expression<'a>,
    ) -> Operand {
        let left_op = if let Some(&id) = self.expr_map.get(left) {
            id
        } else {
            let dest = left.accept(self);
            self.expr_map.insert(left, dest);
            dest
        };

        let right_op = if let Some(&id) = self.expr_map.get(right) {
            id
        } else {
            let dest = right.accept(self);
            self.expr_map.insert(right, dest);
            dest
        };

        // TODO: if string concatenation is allowed this has to change
        let dest_op = Operand::Variable {
            id: self.get_next_variable_id(),
        };

        self.hir.push(Tac::BinExpression {
            left: left_op,
            op: BinaryOperator::from(op),
            right: right_op,
            dest: dest_op,
        });
        self.expr_map.insert(left, dest_op);

        dest_op
    }
}

impl<'a> ast::StatementVisitor<'a> for Builder<'a> {
    fn visit_let(&mut self, variable: &'a str, expression: &ast::Expression<'a>) {
        let dest = self.visit_variable(variable);
        let src = expression.accept(self);

        self.hir.push(Tac::Copy { src, dest });
    }

    fn visit_print(&mut self, content: &'a [&'a ast::Expression<'a>]) {
        // TODO: maybe print all together? How?
        for item in content {
            let operand = item.accept(self);
            self.hir.push(Tac::Param { operand });

            match operand {
                Operand::Variable { .. } | Operand::NumberLiteral { .. } => {
                    self.hir.push(Tac::ExternCall {
                        label: PRINT_VAL_LABEL,
                    });
                }
                Operand::IndirectNumberLiteral { .. } | Operand::IndirectVariable { .. } => {
                    self.hir.push(Tac::ExternCall {
                        label: PRINT_PTR_LABEL,
                    });
                }
            }
        }
    }

    fn visit_input(&mut self, prompt: Option<&'a ast::Expression<'a>>, variable: &'a str) {
        if let Some(prompt) = prompt {
            let prompt = prompt.accept(self);
            self.hir.push(Tac::Param { operand: prompt });

            match prompt {
                Operand::Variable { .. } | Operand::NumberLiteral { .. } => {
                    self.hir.push(Tac::ExternCall {
                        label: PRINT_VAL_LABEL,
                    });
                }
                Operand::IndirectNumberLiteral { .. } | Operand::IndirectVariable { .. } => {
                    self.hir.push(Tac::ExternCall {
                        label: PRINT_PTR_LABEL,
                    });
                }
            }
        }

        let dest = self.visit_variable(variable);
        self.hir.push(Tac::Param { operand: dest });

        match dest {
            Operand::Variable { .. } | Operand::NumberLiteral { .. } => {
                self.hir.push(Tac::ExternCall {
                    label: INPUT_VAL_LABEL,
                });
            }
            Operand::IndirectNumberLiteral { .. } | Operand::IndirectVariable { .. } => {
                self.hir.push(Tac::ExternCall {
                    label: INPUT_PTR_LABEL,
                });
            }
        }
    }

    fn visit_goto(&mut self, line_number: u32) {
        self.goto_list.push(self.hir.len());

        self.hir.push(Tac::Goto { label: line_number });
    }

    fn visit_for(
        &mut self,
        variable: &'a str,
        from: &ast::Expression<'a>,
        to: &ast::Expression<'a>,
        step: Option<&'a ast::Expression<'a>>,
    ) {
        let index = self.visit_variable(variable);
        let from = from.accept(self);
        self.hir.push(Tac::Copy {
            src: from,
            dest: index,
        });

        let to = to.accept(self);

        let info = ForInfo {
            begin_label: self.get_next_label(),
            end_label: self.get_next_label(),
            step,
        };

        self.hir.push(Tac::Label {
            id: info.begin_label,
        });

        self.hir.push(Tac::If {
            op: BinaryOperator::Ge,
            left: index,
            right: to,
            label: info.end_label,
        });

        self.for_stack.push(info);
    }

    fn visit_next(&mut self, variable: &'a str) {
        let index = self.visit_variable(variable);
        let info = self.for_stack.pop().unwrap();

        if let Some(step) = info.step {
            let step = step.accept(self);
            self.hir.push(Tac::BinExpression {
                left: index,
                op: BinaryOperator::Add,
                right: step,
                dest: index,
            });
        } else {
            // Add 1 to the index variable
            self.hir.push(Tac::BinExpression {
                left: index,
                op: BinaryOperator::Add,
                right: Operand::NumberLiteral { value: 1 },
                dest: index,
            });
        }

        self.hir.push(Tac::Goto {
            label: info.begin_label,
        });
        self.hir.push(Tac::Label { id: info.end_label });
    }

    fn visit_end(&mut self) {
        self.hir.push(Tac::ExternCall { label: EXIT_LABEL });
    }

    fn visit_gosub(&mut self, line_number: u32) {
        self.goto_list.push(self.hir.len());

        self.hir.push(Tac::Call { label: line_number });
    }

    fn visit_return(&mut self) {
        self.hir.push(Tac::Return);
    }

    // FIXME: horrible branching
    fn visit_if(
        &mut self,
        condition: &ast::Expression<'a>,
        then: &'a ast::Statement<'a>,
        else_: Option<&'a ast::Statement<'a>>,
    ) {
        let label = self.get_next_label();

        let if_ = match condition {
            ast::Expression::NumberLiteral(val) => {
                let literal = self.visit_number_literal(*val);
                Tac::If {
                    op: BinaryOperator::Ne,
                    left: literal,
                    right: Operand::NumberLiteral { value: 0 },
                    label,
                }
            }
            ast::Expression::StringLiteral(literal) => {
                // FIXME: is this correct? Probably not
                let literal = self.visit_string_literal(literal);
                Tac::If {
                    op: BinaryOperator::Ne,
                    left: literal,
                    right: Operand::NumberLiteral { value: 0 },
                    label,
                }
            }
            ast::Expression::Variable(var) => {
                let var = self.visit_variable(var);
                Tac::If {
                    op: BinaryOperator::Ne,
                    left: var,
                    right: Operand::NumberLiteral { value: 0 },
                    label,
                }
            }
            ast::Expression::Binary { left, op, right } => {
                let left = left.accept(self);
                let right = right.accept(self);

                Tac::If {
                    op: Self::negate_comparison(BinaryOperator::from(*op)),
                    left,
                    right,
                    label,
                }
            }
        };

        self.hir.push(if_);

        then.accept(self);

        if let Some(else_) = else_ {
            let else_label = self.get_next_label();
            self.hir.push(Tac::Goto { label: else_label });
            self.hir.push(Tac::Label { id: label });
            else_.accept(self);
            self.hir.push(Tac::Label { id: else_label });
        } else {
            self.hir.push(Tac::Label { id: label });
        }
    }

    fn visit_seq(&mut self, statements: &'a [ast::Statement<'a>]) {
        for stmt in statements {
            stmt.accept(self);
        }
    }
}

impl<'a> ast::ProgramVisitor<'a> for Builder<'a> {
    fn visit_program(&mut self, program: &'a ast::Program<'a>) {
        for (&line_number, stmt) in program.iter() {
            self.line_to_hir_map
                .insert(line_number as usize, self.hir.len());
            stmt.accept(self);
        }

        let mut i = 0;

        while i < self.goto_list.len() {
            let goto_idx = self.goto_list[i];

            // TODO: check there is already a label
            let new_label = {
                let line = match &self.hir[goto_idx] {
                    Tac::Goto { label: line } | Tac::Call { label: line } => *line as usize,
                    Tac::ExternCall { label: _ } => {
                        // IGNORE EXTERN CALLS
                        i += 1;
                        continue;
                    }
                    _ => unreachable!("Invalid goto position: {}", self.hir[goto_idx]),
                };

                // Add label before jump position
                let new_label_pos = *self.line_to_hir_map.get(&line).unwrap();
                let new_label = self.get_next_label();

                self.hir.insert(new_label_pos, Tac::Label { id: new_label });

                for j in i..self.goto_list.len() {
                    if self.goto_list[j] >= new_label_pos {
                        self.goto_list[j] += 1;
                    }
                }

                new_label
            };

            let goto_idx = self.goto_list[i];

            match &self.hir[goto_idx] {
                Tac::Goto { .. } => self.hir[goto_idx] = Tac::Goto { label: new_label },
                Tac::Call { .. } => self.hir[goto_idx] = Tac::Call { label: new_label },
                _ => unreachable!("Invalid goto position: {}", self.hir[goto_idx]),
            }

            i += 1;
        }
    }
}
