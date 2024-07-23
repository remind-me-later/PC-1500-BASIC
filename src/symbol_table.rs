use std::collections::HashMap;

use crate::ast::{Ast, AstVisitor};

pub enum VariableType {
    Int,
    String,
}

pub struct VariableInfo<'a> {
    name: &'a str,
    ty: VariableType,
}

pub struct SymbolTable<'a> {
    symbols: HashMap<&'a str, VariableInfo<'a>>,
}

impl<'a> SymbolTable<'a> {
    pub fn new() -> Self {
        SymbolTable {
            symbols: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: &'a str) {
        let ty = if name.ends_with('$') {
            VariableType::String
        } else {
            VariableType::Int
        };
        self.symbols.insert(name, VariableInfo { name, ty });
    }

    pub fn lookup(&self, name: &'a str) -> Option<&VariableInfo<'a>> {
        self.symbols.get(name)
    }
}

pub struct SymbolTableBuilderVisitor<'a> {
    symbol_table: SymbolTable<'a>,
}

impl<'a> SymbolTableBuilderVisitor<'a> {
    pub fn new() -> Self {
        SymbolTableBuilderVisitor {
            symbol_table: SymbolTable::new(),
        }
    }

    pub fn build(mut self, ast: &'a Ast<'a>) -> SymbolTable<'a> {
        ast.accept(&mut self);
        self.symbol_table
    }
}

impl<'a> AstVisitor<'a> for SymbolTableBuilderVisitor<'a> {
    fn visit_variable(&mut self, name: &'a str) {
        self.symbol_table.insert(name)
    }

    fn visit_literal(&mut self, _: i32) {}

    fn visit_binary_op(
        &mut self,
        left: &crate::ast::Expression<'a>,
        _: crate::ast::BinaryOperator,
        right: &crate::ast::Expression<'a>,
    ) {
        left.accept(self);
        right.accept(self);
    }

    fn visit_let(&mut self, variable: &'a str, expression: &crate::ast::Expression<'a>) {
        self.symbol_table.insert(variable);
        expression.accept(self);
    }

    fn visit_print(&mut self, content: &[crate::ast::PrintContent<'a>]) {
        for item in content {
            match item {
                crate::ast::PrintContent::StringLiteral(_) => {}
                crate::ast::PrintContent::Expression(expr) => expr.accept(self),
            }
        }
    }

    fn visit_input(&mut self, _: Option<&str>, variable: &'a str) {
        self.symbol_table.insert(variable);
    }

    fn visit_goto(&mut self, _: u32, _: Option<&'a Ast<'a>>) {}

    fn visit_for(
        &mut self,
        variable: &'a str,
        from: &crate::ast::Expression<'a>,
        to: &crate::ast::Expression<'a>,
        step: Option<&crate::ast::Expression<'a>>,
    ) {
        self.symbol_table.insert(variable);
        from.accept(self);
        to.accept(self);
        if let Some(step) = step {
            step.accept(self);
        }
    }

    fn visit_next(&mut self, variable: &'a str) {
        self.symbol_table.insert(variable);
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
        condition.accept(self);
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
