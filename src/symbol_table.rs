use std::collections::HashMap;

use crate::dag::{ExpressionVisitor, Program, ProgramVisitor, Statement, StatementVisitor};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Ty {
    Int,
    String,
}

impl std::fmt::Display for Ty {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Ty::Int => write!(f, "INT"),
            Ty::String => write!(f, "STRING"),
        }
    }
}

pub struct VariableInfo<'a> {
    name: &'a str,
    ty: Ty,
}

impl<'a> VariableInfo<'a> {
    pub fn name(&self) -> &'a str {
        self.name
    }

    pub fn ty(&self) -> Ty {
        self.ty
    }
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
            Ty::String
        } else {
            Ty::Int
        };
        self.symbols.insert(name, VariableInfo { name, ty });
    }

    pub fn lookup(&self, name: &'a str) -> Option<&VariableInfo<'a>> {
        self.symbols.get(name)
    }
}

impl std::fmt::Display for SymbolTable<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (name, info) in &self.symbols {
            writeln!(f, "{} is {}", name, info.ty())?;
        }
        Ok(())
    }
}

pub struct SymbolTableBuilderVisitor<'a> {
    program: &'a Program<'a>,
    symbol_table: SymbolTable<'a>,
}

impl<'a> SymbolTableBuilderVisitor<'a> {
    pub fn new(program: &'a Program<'a>) -> Self {
        SymbolTableBuilderVisitor {
            program,
            symbol_table: SymbolTable::new(),
        }
    }

    pub fn build(mut self) -> SymbolTable<'a> {
        self.program.accept(&mut self);
        self.symbol_table
    }
}

impl<'a> ExpressionVisitor<'a> for SymbolTableBuilderVisitor<'a> {
    fn visit_variable(&mut self, name: &'a str) {
        self.symbol_table.insert(name)
    }

    fn visit_number_literal(&mut self, _: i32) {}

    fn visit_binary_op(
        &mut self,
        left: &crate::dag::Expression<'a>,
        _: crate::dag::BinaryOperator,
        right: &crate::dag::Expression<'a>,
    ) {
        left.accept(self);
        right.accept(self);
    }
}

impl<'a> StatementVisitor<'a> for SymbolTableBuilderVisitor<'a> {
    fn visit_let(&mut self, variable: &'a str, expression: &crate::dag::Expression<'a>) {
        self.symbol_table.insert(variable);
        expression.accept(self);
    }

    fn visit_print(&mut self, content: &[crate::dag::PrintContent<'a>]) {
        for item in content {
            match item {
                crate::dag::PrintContent::StringLiteral(_) => {}
                crate::dag::PrintContent::Expression(expr) => expr.accept(self),
            }
        }
    }

    fn visit_input(&mut self, _: Option<&str>, variable: &'a str) {
        self.symbol_table.insert(variable);
    }

    fn visit_goto(&mut self, _: u32, _: Option<&'a Statement<'a>>) {}

    fn visit_for(
        &mut self,
        variable: &'a str,
        from: &crate::dag::Expression<'a>,
        to: &crate::dag::Expression<'a>,
        step: Option<&crate::dag::Expression<'a>>,
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

    fn visit_gosub(&mut self, _: u32, _: Option<&'a Statement<'a>>) {}

    fn visit_return(&mut self) {}

    fn visit_if(
        &mut self,
        condition: &crate::dag::Expression<'a>,
        then: &'a Statement<'a>,
        else_: Option<&'a Statement<'a>>,
    ) {
        condition.accept(self);
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

impl<'a> ProgramVisitor<'a> for SymbolTableBuilderVisitor<'a> {
    fn visit_program(&mut self, program: &'a Program<'a>) {
        for statement in program.values() {
            statement.accept(self);
        }
    }
}
