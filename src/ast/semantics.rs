use super::{
    node::UnaryOperator, BinaryOperator, Expression, ExpressionVisitor, Program, ProgramVisitor,
    Statement, StatementVisitor,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ty {
    Int,
    String,
}

impl std::fmt::Display for Ty {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Ty::Int => write!(f, "INT"),
            Ty::String => write!(f, "STR"),
        }
    }
}

pub struct SemanticChecker<'a> {
    program: &'a Program,
    errors: Vec<String>,
    // symbol_table: &'a SymbolTable<'a>,
    for_stack: Vec<&'a str>,
}

impl<'a> SemanticChecker<'a> {
    pub fn new(program: &'a Program) -> Self {
        SemanticChecker {
            errors: Vec::new(),
            for_stack: Vec::new(),
            program,
            // symbol_table,
        }
    }

    pub fn check(mut self) -> Result<(), Vec<String>> {
        self.program.accept(&mut self);
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors)
        }
    }

    fn get_ty(&self, name: &'a str) -> Ty {
        if name.ends_with("$") {
            Ty::String
        } else {
            Ty::Int
        }
    }
}

impl<'a> ExpressionVisitor<'a, Ty> for SemanticChecker<'a> {
    fn visit_variable(&mut self, name: &'a str) -> Ty {
        self.get_ty(name)
    }

    fn visit_number_literal(&mut self, _: i32) -> Ty {
        Ty::Int
    }

    fn visit_unary_op(&mut self, op: UnaryOperator, operand: &'a Expression) -> Ty {
        let operand_ty = operand.accept(self);
        match op {
            UnaryOperator::Not => {
                if operand_ty != Ty::Int {
                    self.errors
                        .push("NOT operand must be an integer".to_owned());
                }
            }
            UnaryOperator::Plus | UnaryOperator::Minus => {
                if operand_ty != Ty::Int {
                    self.errors
                        .push("Unary plus/minus operand must be an integer".to_owned());
                }
            }
        }

        Ty::Int
    }

    fn visit_binary_op(
        &mut self,
        left: &'a Expression,
        op: BinaryOperator,
        right: &'a Expression,
    ) -> Ty {
        let left_ty = left.accept(self);
        let right_ty = right.accept(self);

        if left_ty != right_ty {
            self.errors.push(format!(
                "Type mismatch: left operand is {}, right operand is {}",
                left_ty, right_ty
            ));
        }

        match op {
            BinaryOperator::Add
            | BinaryOperator::Sub
            | BinaryOperator::Mul
            | BinaryOperator::Div
            | BinaryOperator::And
            | BinaryOperator::Or => {
                if left_ty != Ty::Int {
                    self.errors
                        .push("Arithmetic operands must be integers".to_owned());
                }
            }
            BinaryOperator::Eq
            | BinaryOperator::Ne
            | BinaryOperator::Lt
            | BinaryOperator::Le
            | BinaryOperator::Gt
            | BinaryOperator::Ge => {
                // Itegers and string are comparable
                // in the case of strings, the comparison is lexicographical
            }
        }

        Ty::Int
    }

    fn visit_string_literal(&mut self, _: &'a str) -> Ty {
        Ty::String
    }
}

impl<'a> StatementVisitor<'a> for SemanticChecker<'a> {
    fn visit_let(&mut self, variable: &'a str, expression: &'a Expression) {
        let expr_ty = expression.accept(self);
        let expected_ty = self.get_ty(variable);
        if expr_ty != expected_ty {
            self.errors.push(format!(
                "Type mismatch: variable {} is {}, expression is {}",
                variable, expected_ty, expr_ty
            ));
        }
    }

    fn visit_print(&mut self, content: &'a [Expression]) {
        for item in content {
            item.accept(self);
        }
    }

    fn visit_input(&mut self, _: Option<&'a Expression>, _: &'a str) {
        // TODO: check prompt is string? Are integer prompts allowed?
    }

    fn visit_goto(&mut self, line_number: u32) {
        let to_node = self.program.lookup_line(line_number);
        if to_node.is_none() {
            self.errors
                .push(format!("GOTO to undefined line {}", line_number));
        }
    }

    fn visit_for(
        &mut self,
        variable: &'a str,
        from: &'a Expression,
        to: &'a Expression,
        step: Option<&'a Expression>,
    ) {
        let var_ty = self.get_ty(variable);

        if var_ty != Ty::Int {
            self.errors
                .push("Loop variable must be an integer".to_owned());
        }

        let from_ty = from.accept(self);
        let to_ty = to.accept(self);

        if from_ty != Ty::Int || to_ty != Ty::Int {
            self.errors.push("Loop bounds must be integers".to_owned());
        }

        if let Some(step) = step {
            let step_ty = step.accept(self);
            if step_ty != Ty::Int {
                self.errors.push("Loop step must be an integer".to_owned());
            }
        }

        self.for_stack.push(variable);
    }

    fn visit_next(&mut self, variable: &'a str) {
        let var_ty = self.get_ty(variable);
        if var_ty != Ty::Int {
            self.errors
                .push("Loop variable must be an integer".to_owned());
        }

        if let Some(last) = self.for_stack.pop() {
            if last != variable {
                self.errors.push(
                    "NEXT variable: ".to_owned()
                        + variable
                        + " does not match FOR variable: "
                        + last,
                );
            }
        } else {
            self.errors.push("NEXT without matching FOR".to_owned());
        }
    }

    fn visit_end(&mut self) {}

    fn visit_gosub(&mut self, line_number: u32) {
        let to_node = self.program.lookup_line(line_number);
        if to_node.is_none() {
            self.errors
                .push(format!("GOSUB to undefined line {}", line_number));
        }
    }

    fn visit_return(&mut self) {}

    fn visit_if(
        &mut self,
        condition: &'a Expression,
        then: &'a Statement,
        else_: Option<&'a Statement>,
    ) {
        let condition_ty = condition.accept(self);
        if condition_ty != Ty::Int {
            self.errors.push("Condition must be an integer".to_owned());
        }

        then.accept(self);
        if let Some(else_) = else_ {
            else_.accept(self);
        }
    }

    fn visit_seq(&mut self, statements: &'a [Statement]) {
        for statement in statements {
            statement.accept(self);
        }
    }

    fn visit_rem(&mut self, _: &'a str) {}
}

impl<'a> ProgramVisitor<'a> for SemanticChecker<'a> {
    fn visit_program(&mut self, program: &'a Program) {
        for statement in program.values() {
            statement.accept(self);
        }
    }
}
