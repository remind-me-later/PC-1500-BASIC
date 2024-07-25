use std::{collections::HashMap, ptr};

use crate::{
    ast::{self, ExpressionVisitor},
    hir::{INPUT_PTR_LABEL, PRINT_PTR_LABEL},
};

use super::{
    BinaryOperator, Expression, Hir, Operand, Program, END_OF_BUILTIN_LABELS, INPUT_VAL_LABEL,
    PRINT_VAL_LABEL,
};

struct ForInfo<'a> {
    begin_label: u32,
    end_label: u32,
    step: Option<&'a ast::Expression<'a>>,
}

pub struct HirBuilder<'a> {
    hir: Vec<Hir>,

    program: &'a ast::Program<'a>,

    var_map: HashMap<*const str, Operand>,
    expr_map: HashMap<*const ast::Expression<'a>, Operand>,

    str_map: HashMap<*const str, usize>,
    str_literals: Vec<String>,

    line_to_hir_map: HashMap<usize, usize>,

    for_stack: Vec<ForInfo<'a>>,
    goto_list: Vec<usize>,

    next_variable: u32,
    next_label: u32,
}

impl<'a> HirBuilder<'a> {
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

    fn insert_str_literal(&mut self, s: &str) -> u32 {
        // TODO: check overflows
        if let Some(&id) = self.str_map.get(&ptr::from_ref(s)) {
            id as u32
        } else {
            let id = self.str_literals.len();
            self.str_literals.push(s.to_string());
            self.str_map.insert(ptr::from_ref(s), id);
            id as u32
        }
    }
}

impl<'a> ast::ExpressionVisitor<'a, Operand> for HirBuilder<'a> {
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
        if let Some(&id) = self.var_map.get(&ptr::from_ref(variable)) {
            id
        } else {
            let id = self.get_next_variable_id();

            let var = if variable.trim().ends_with("$") {
                Operand::IndirectVariable { id }
            } else {
                Operand::Variable { id }
            };

            self.var_map.insert(ptr::from_ref(variable), var);

            var
        }
    }

    fn visit_binary_op(
        &mut self,
        left: &'a ast::Expression<'a>,
        op: ast::BinaryOperator,
        right: &'a ast::Expression<'a>,
    ) -> Operand {
        let left_op = if let Some(&id) = self.expr_map.get(&ptr::from_ref(left)) {
            id
        } else {
            let dest = left.accept(self);
            self.expr_map.insert(ptr::from_ref(left), dest);
            dest
        };

        let right_op = if let Some(&id) = self.expr_map.get(&ptr::from_ref(right)) {
            id
        } else {
            let dest = right.accept(self);
            self.expr_map.insert(ptr::from_ref(right), dest);
            dest
        };

        // TODO: if string concatenation is allowed this has to change
        let dest_op = Operand::Variable {
            id: self.get_next_variable_id(),
        };

        let expr = match op {
            ast::BinaryOperator::Add => Expression {
                left: left_op,
                op: BinaryOperator::Add,
                right: right_op,
                dest: dest_op,
            },
            ast::BinaryOperator::Sub => Expression {
                left: left_op,
                op: BinaryOperator::Sub,
                right: right_op,
                dest: dest_op,
            },
            ast::BinaryOperator::Mul => Expression {
                left: left_op,
                op: BinaryOperator::Mul,
                right: right_op,
                dest: dest_op,
            },
            ast::BinaryOperator::Div => Expression {
                left: left_op,
                op: BinaryOperator::Div,
                right: right_op,
                dest: dest_op,
            },
            ast::BinaryOperator::And => Expression {
                left: left_op,
                op: BinaryOperator::And,
                right: right_op,
                dest: dest_op,
            },
            ast::BinaryOperator::Or => Expression {
                left: left_op,
                op: BinaryOperator::Or,
                right: right_op,
                dest: dest_op,
            },
            ast::BinaryOperator::Eq => Expression {
                left: left_op,
                op: BinaryOperator::Eq,
                right: right_op,
                dest: dest_op,
            },
            ast::BinaryOperator::Ne => Expression {
                left: left_op,
                op: BinaryOperator::Ne,
                right: right_op,
                dest: dest_op,
            },
            ast::BinaryOperator::Lt => Expression {
                left: left_op,
                op: BinaryOperator::Lt,
                right: right_op,
                dest: dest_op,
            },
            ast::BinaryOperator::Le => Expression {
                left: left_op,
                op: BinaryOperator::Le,
                right: right_op,
                dest: dest_op,
            },
            ast::BinaryOperator::Gt => Expression {
                left: left_op,
                op: BinaryOperator::Gt,
                right: right_op,
                dest: dest_op,
            },
            ast::BinaryOperator::Ge => Expression {
                left: left_op,
                op: BinaryOperator::Ge,
                right: right_op,
                dest: dest_op,
            },
        };

        self.hir.push(Hir::Expression(expr));
        self.expr_map.insert(ptr::from_ref(left), dest_op);

        dest_op
    }
}

impl<'a> ast::StatementVisitor<'a> for HirBuilder<'a> {
    fn visit_let(&mut self, variable: &'a str, expression: &ast::Expression<'a>) {
        let dest = self.visit_variable(variable);
        let src = expression.accept(self);

        self.hir.push(Hir::Copy { src, dest });
    }

    fn visit_print(&mut self, content: &'a [&'a ast::Expression<'a>]) {
        // TODO: maybe print all together? How?
        for item in content {
            let operand = item.accept(self);
            self.hir.push(Hir::Param { operand });

            match operand {
                Operand::Variable { .. } | Operand::NumberLiteral { .. } => {
                    self.hir.push(Hir::Call {
                        label: PRINT_VAL_LABEL,
                    });
                }
                Operand::IndirectNumberLiteral { .. } | Operand::IndirectVariable { .. } => {
                    self.hir.push(Hir::Call {
                        label: PRINT_PTR_LABEL,
                    });
                }
            }
        }
    }

    fn visit_input(&mut self, prompt: Option<&'a ast::Expression<'a>>, variable: &'a str) {
        if let Some(prompt) = prompt {
            let prompt = prompt.accept(self);
            self.hir.push(Hir::Param { operand: prompt });

            match prompt {
                Operand::Variable { .. } | Operand::NumberLiteral { .. } => {
                    self.hir.push(Hir::Call {
                        label: PRINT_VAL_LABEL,
                    });
                }
                Operand::IndirectNumberLiteral { .. } | Operand::IndirectVariable { .. } => {
                    self.hir.push(Hir::Call {
                        label: PRINT_PTR_LABEL,
                    });
                }
            }
        }

        let dest = self.visit_variable(variable);
        self.hir.push(Hir::Param { operand: dest });

        match dest {
            Operand::Variable { .. } | Operand::NumberLiteral { .. } => {
                self.hir.push(Hir::Call {
                    label: INPUT_VAL_LABEL,
                });
            }
            Operand::IndirectNumberLiteral { .. } | Operand::IndirectVariable { .. } => {
                self.hir.push(Hir::Call {
                    label: INPUT_PTR_LABEL,
                });
            }
        }
    }

    fn visit_goto(&mut self, line_number: u32) {
        self.goto_list.push(self.hir.len());
        self.hir.push(Hir::Goto { label: line_number });
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
        self.hir.push(Hir::Copy {
            src: from,
            dest: index,
        });

        let to = to.accept(self);
        let cmp_dest = Operand::Variable {
            id: self.get_next_variable_id(),
        };

        let info = ForInfo {
            begin_label: self.get_next_label(),
            end_label: self.get_next_label(),
            step,
        };

        self.hir.push(Hir::Label {
            id: info.begin_label,
        });

        self.hir.push(Hir::Expression(Expression {
            left: index,
            op: BinaryOperator::Ge,
            right: to,
            dest: cmp_dest,
        }));

        self.hir.push(Hir::If {
            condition: cmp_dest,
            label: info.end_label,
        });

        self.for_stack.push(info);
    }

    fn visit_next(&mut self, variable: &'a str) {
        let index = self.visit_variable(variable);
        let info = self.for_stack.pop().unwrap();

        if let Some(step) = info.step {
            let step = step.accept(self);
            self.hir.push(Hir::Expression(Expression {
                left: index,
                op: BinaryOperator::Add,
                right: step,
                dest: index,
            }));
        } else {
            // Add 1 to the index variable
            self.hir.push(Hir::Expression(Expression {
                left: index,
                op: BinaryOperator::Add,
                right: Operand::NumberLiteral { value: 1 },
                dest: index,
            }));
        }

        self.hir.push(Hir::Goto {
            label: info.begin_label,
        });
        self.hir.push(Hir::Label { id: info.end_label });
    }

    fn visit_end(&mut self) {}

    fn visit_gosub(&mut self, line_number: u32) {
        self.goto_list.push(self.hir.len());
        self.hir.push(Hir::GoSub { label: line_number });
    }

    fn visit_return(&mut self) {
        self.hir.push(Hir::Return);
    }

    fn visit_if(
        &mut self,
        condition: &ast::Expression<'a>,
        then: &'a ast::Statement<'a>,
        else_: Option<&'a ast::Statement<'a>>,
    ) {
        let cond = condition.accept(self);
        let neg_cond = Operand::Variable {
            id: self.get_next_variable_id(),
        };
        self.hir.push(Hir::Expression(Expression {
            left: cond,
            op: BinaryOperator::Eq,
            right: Operand::NumberLiteral { value: 0 },
            dest: neg_cond,
        }));

        let label = self.get_next_label();

        self.hir.push(Hir::If {
            condition: neg_cond,
            label,
        });

        then.accept(self);

        self.hir.push(Hir::Label { id: label });

        if let Some(else_) = else_ {
            else_.accept(self);
        }
    }

    fn visit_seq(&mut self, statements: &'a [ast::Statement<'a>]) {
        for stmt in statements {
            stmt.accept(self);
        }
    }
}

impl<'a> ast::ProgramVisitor<'a> for HirBuilder<'a> {
    fn visit_program(&mut self, program: &'a ast::Program<'a>) {
        for (&line_number, stmt) in program.iter() {
            self.line_to_hir_map
                .insert(line_number as usize, self.hir.len());
            stmt.accept(self);
        }

        let mut offset = 0;

        while let Some(og_goto) = self.goto_list.pop() {
            let goto = og_goto + offset;
            let line = if let Hir::Goto { label: line } = &self.hir[goto] {
                *line as usize
            } else if let Hir::GoSub { label: line } = &self.hir[goto] {
                *line as usize
            } else {
                unreachable!("Invalid goto position");
            };

            // Add label before jump position
            let new_label_pos = *self.line_to_hir_map.get(&line).unwrap() + offset;

            // check there is already a label
            // TODO: ugly as fuck
            let new_label = if new_label_pos > 1 {
                if let Hir::Label { id } = &self.hir[new_label_pos - 1] {
                    *id
                } else {
                    let new_label = self.get_next_label();

                    self.hir.insert(new_label_pos, Hir::Label { id: new_label });

                    offset += 1;

                    new_label
                }
            } else {
                let new_label = self.get_next_label();

                self.hir.insert(new_label_pos, Hir::Label { id: new_label });

                offset += 1;

                new_label
            };

            let goto = og_goto + offset;

            if let Hir::Goto { .. } = &self.hir[goto] {
                self.hir[goto] = Hir::Goto { label: new_label };
            } else if let Hir::GoSub { .. } = &self.hir[goto] {
                self.hir[goto] = Hir::GoSub { label: new_label };
            } else {
                unreachable!("Invalid goto position");
            }
        }
    }
}
