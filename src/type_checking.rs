use crate::{
    ast::{Ast, AstVisitor, ExpressionVisitor},
    symbol_table::{SymbolTable, Ty},
};

pub struct TypeCheckVisitor<'a> {
    errors: Vec<String>,
    symbol_table: &'a SymbolTable<'a>,
}

impl<'a> TypeCheckVisitor<'a> {
    pub fn new(symbol_table: &'a SymbolTable<'a>) -> Self {
        TypeCheckVisitor {
            errors: Vec::new(),
            symbol_table,
        }
    }

    pub fn check(mut self, ast: &'a Ast<'a>) -> Result<(), Vec<String>> {
        ast.accept(&mut self);
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors)
        }
    }
}

impl<'a> ExpressionVisitor<'a, Ty> for TypeCheckVisitor<'a> {
    fn visit_variable(&mut self, name: &'a str) -> Ty {
        self.symbol_table.lookup(name).unwrap().ty()
    }

    fn visit_number_literal(&mut self, _: i32) -> Ty {
        Ty::Int
    }

    fn visit_binary_op(
        &mut self,
        left: &crate::ast::Expression<'a>,
        _: crate::ast::BinaryOperator,
        right: &crate::ast::Expression<'a>,
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

impl<'a> AstVisitor<'a> for TypeCheckVisitor<'a> {
    fn visit_let(&mut self, variable: &'a str, expression: &crate::ast::Expression<'a>) {
        let expr_ty = expression.accept(self);
        let expected_ty = self.symbol_table.lookup(variable).unwrap().ty();
        if expr_ty != expected_ty {
            self.errors.push(format!(
                "Type mismatch: variable {} is {}, expression is {}",
                variable, expected_ty, expr_ty
            ));
        }
    }

    fn visit_print(&mut self, content: &[crate::ast::PrintContent<'a>]) {
        for item in content {
            match item {
                crate::ast::PrintContent::StringLiteral(_) => {}
                crate::ast::PrintContent::Expression(expr) => {
                    expr.accept(self);
                }
            }
        }
    }

    fn visit_input(&mut self, _: Option<&str>, _: &'a str) {}

    fn visit_goto(&mut self, _: u32, _: Option<&'a Ast<'a>>) {}

    fn visit_for(
        &mut self,
        variable: &'a str,
        from: &crate::ast::Expression<'a>,
        to: &crate::ast::Expression<'a>,
        step: Option<&crate::ast::Expression<'a>>,
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
    }

    fn visit_next(&mut self, variable: &'a str) {
        let var_ty = self.symbol_table.lookup(variable).unwrap().ty();
        if var_ty != Ty::Int {
            self.errors
                .push("Loop variable must be an integer".to_string());
        }
    }

    fn visit_end(&mut self) {}

    fn visit_gosub(&mut self, _: u32, _: Option<&'a Ast<'a>>) {}

    fn visit_return(&mut self) {}

    fn visit_if(
        &mut self,
        condition: &crate::ast::Expression<'a>,
        then: &'a Ast<'a>,
        else_: Option<&'a Ast<'a>>,
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

    fn visit_seq(&mut self, statements: &'a [Ast<'a>]) {
        for statement in statements {
            statement.accept(self);
        }
    }

    fn visit_program(&mut self, lines: &'a std::collections::BTreeMap<u32, Ast<'a>>) {
        for statement in lines.values() {
            statement.accept(self);
        }
    }
}
