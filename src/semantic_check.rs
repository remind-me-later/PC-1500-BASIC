use crate::{
    dag::{ExpressionVisitor, Program, ProgramVisitor, Statement, StatementVisitor},
    symbol_table::{SymbolTable, Ty},
};

pub struct SemanticCheckVisitor<'a> {
    program: &'a Program<'a>,
    errors: Vec<String>,
    symbol_table: &'a SymbolTable<'a>,
    for_stack: Vec<&'a str>,
}

impl<'a> SemanticCheckVisitor<'a> {
    pub fn new(symbol_table: &'a SymbolTable<'a>, program: &'a Program<'a>) -> Self {
        SemanticCheckVisitor {
            errors: Vec::new(),
            for_stack: Vec::new(),
            program,
            symbol_table,
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
}

impl<'a> ExpressionVisitor<'a, Ty> for SemanticCheckVisitor<'a> {
    fn visit_variable(&mut self, name: &'a str) -> Ty {
        self.symbol_table.lookup(name).unwrap().ty()
    }

    fn visit_number_literal(&mut self, _: i32) -> Ty {
        Ty::Int
    }

    fn visit_binary_op(
        &mut self,
        left: &crate::dag::Expression<'a>,
        _: crate::dag::BinaryOperator,
        right: &crate::dag::Expression<'a>,
    ) -> Ty {
        let left_ty = left.accept(self);
        let right_ty = right.accept(self);

        if left_ty != right_ty {
            self.errors.push(format!(
                "Type mismatch: left operand is {}, right operand is {}",
                left_ty, right_ty
            ));
        }

        if left_ty == Ty::String {
            self.errors
                .push("Cannot perform arithmetic on strings".to_string());
        }

        Ty::Int
    }
}

impl<'a> StatementVisitor<'a> for SemanticCheckVisitor<'a> {
    fn visit_let(&mut self, variable: &'a str, expression: &crate::dag::Expression<'a>) {
        let expr_ty = expression.accept(self);
        let expected_ty = self.symbol_table.lookup(variable).unwrap().ty();
        if expr_ty != expected_ty {
            self.errors.push(format!(
                "Type mismatch: variable {} is {}, expression is {}",
                variable, expected_ty, expr_ty
            ));
        }
    }

    fn visit_print(&mut self, content: &[crate::dag::PrintContent<'a>]) {
        for item in content {
            match item {
                crate::dag::PrintContent::StringLiteral(_) => {}
                crate::dag::PrintContent::Expression(expr) => {
                    expr.accept(self);
                }
            }
        }
    }

    fn visit_input(&mut self, _: Option<&str>, _: &'a str) {}

    fn visit_goto(&mut self, line_number: u32, _: Option<&'a Statement<'a>>) {
        let to_node = self.program.lookup_line(line_number);
        if to_node.is_none() {
            self.errors
                .push(format!("GOTO to undefined line {}", line_number));
        }
    }

    fn visit_for(
        &mut self,
        variable: &'a str,
        from: &crate::dag::Expression<'a>,
        to: &crate::dag::Expression<'a>,
        step: Option<&crate::dag::Expression<'a>>,
    ) {
        let var_ty = self.symbol_table.lookup(variable).unwrap().ty();

        if var_ty != Ty::Int {
            self.errors
                .push("Loop variable must be an integer".to_string());
        }

        let from_ty = from.accept(self);
        let to_ty = to.accept(self);

        if from_ty != Ty::Int || to_ty != Ty::Int {
            self.errors.push("Loop bounds must be integers".to_string());
        }

        if let Some(step) = step {
            let step_ty = step.accept(self);
            if step_ty != Ty::Int {
                self.errors.push("Loop step must be an integer".to_string());
            }
        }

        self.for_stack.push(variable);
    }

    fn visit_next(&mut self, variable: &'a str) {
        let var_ty = self.symbol_table.lookup(variable).unwrap().ty();
        if var_ty != Ty::Int {
            self.errors
                .push("Loop variable must be an integer".to_string());
        }

        if let Some(last) = self.for_stack.pop() {
            if last != variable {
                self.errors.push(
                    "NEXT variable: ".to_string()
                        + variable
                        + " does not match FOR variable: "
                        + last,
                );
            }
        } else {
            self.errors.push("NEXT without matching FOR".to_string());
        }
    }

    fn visit_end(&mut self) {}

    fn visit_gosub(&mut self, line_number: u32, _: Option<&'a Statement<'a>>) {
        let to_node = self.program.lookup_line(line_number);
        if to_node.is_none() {
            self.errors
                .push(format!("GOSUB to undefined line {}", line_number));
        }
    }

    fn visit_return(&mut self) {}

    fn visit_if(
        &mut self,
        condition: &crate::dag::Expression<'a>,
        then: &'a Statement<'a>,
        else_: Option<&'a Statement<'a>>,
    ) {
        let condition_ty = condition.accept(self);
        if condition_ty != Ty::Int {
            self.errors.push("Condition must be an integer".to_string());
        }

        then.accept(self);
        if let Some(else_) = else_ {
            else_.accept(self);
        }
    }

    fn visit_seq(&mut self, statements: &'a [Statement<'a>]) {
        for statement in statements {
            statement.accept(self);
        }
    }
}

impl<'a> ProgramVisitor<'a> for SemanticCheckVisitor<'a> {
    fn visit_program(&mut self, program: &'a crate::dag::Program<'a>) {
        for statement in program.values() {
            statement.accept(self);
        }
    }
}
