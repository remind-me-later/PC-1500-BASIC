use crate::{
    ast::{Ast, AstVisitor},
    symbol_table::SymbolTable,
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

impl<'a> AstVisitor<'a> for TypeCheckVisitor<'a> {
    fn visit_variable(&mut self, _: &'a str) {}

    fn visit_literal(&mut self, _: i32) {}

    fn visit_binary_op(
        &mut self,
        left: &crate::ast::Expression<'a>,
        op: crate::ast::BinaryOperator,
        right: &crate::ast::Expression<'a>,
    ) {
        left.accept(self);
        right.accept(self);
    }

    fn visit_let(&mut self, variable: &'a str, expression: &crate::ast::Expression<'a>) {
        todo!()
    }

    fn visit_print(&mut self, content: &[crate::ast::PrintContent<'a>]) {
        todo!()
    }

    fn visit_input(&mut self, prompt: Option<&str>, variable: &'a str) {
        todo!()
    }

    fn visit_goto(&mut self, line_number: u32, to: Option<&'a Ast<'a>>) {
        todo!()
    }

    fn visit_for(
        &mut self,
        variable: &'a str,
        from: &crate::ast::Expression<'a>,
        to: &crate::ast::Expression<'a>,
        step: Option<&crate::ast::Expression<'a>>,
    ) {
        todo!()
    }

    fn visit_next(&mut self, variable: &'a str) {
        todo!()
    }

    fn visit_end(&mut self) {
        todo!()
    }

    fn visit_gosub(&mut self, line_number: u32, to: Option<&'a Ast<'a>>) {
        todo!()
    }

    fn visit_return(&mut self) {
        todo!()
    }

    fn visit_if(
        &mut self,
        condition: &crate::ast::Expression<'a>,
        then: &'a Ast<'a>,
        else_: Option<&'a Ast<'a>>,
    ) {
        todo!()
    }

    fn visit_seq(&mut self, statements: &'a [Ast<'a>]) {
        todo!()
    }

    fn visit_program(&mut self, lines: &'a std::collections::BTreeMap<u32, Ast<'a>>) {
        todo!()
    }
}
