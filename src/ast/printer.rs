use std::marker::PhantomData;

use super::{
    node::{DataItem, LValue, UnaryOperator},
    Expression, ExpressionVisitor, Program, ProgramVisitor, Statement, StatementVisitor,
};

pub struct Printer<'a> {
    output: String,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> Printer<'a> {
    pub fn new() -> Self {
        Printer {
            output: String::new(),
            _phantom: PhantomData,
        }
    }

    pub fn build(self, ast: &'a Program) -> String {
        let mut visitor = Printer::new();
        ast.accept(&mut visitor);
        visitor.output
    }
}

impl<'a> ExpressionVisitor<'a> for Printer<'a> {
    fn visit_number_literal(&mut self, num: i32) {
        self.output.push_str(&num.to_string());
    }

    fn visit_variable(&mut self, variable: &'a LValue) {
        self.output.push_str(variable.to_string().as_str());
    }

    fn visit_unary_op(&mut self, op: UnaryOperator, operand: &'a Expression) {
        self.output.push_str(op.to_string().as_str());
        operand.accept(self);
    }

    fn visit_binary_op(
        &mut self,
        left: &'a Expression,
        op: super::BinaryOperator,
        right: &'a Expression,
    ) {
        self.output.push('(');
        left.accept(self);
        self.output.push(' ');
        self.output.push_str(op.to_string().as_str());
        self.output.push(' ');
        right.accept(self);
        self.output.push(')');
    }

    fn visit_string_literal(&mut self, content: &'a str) {
        self.output.push('"');
        self.output.push_str(content);
        self.output.push('"');
    }
}

impl<'a> StatementVisitor<'a> for Printer<'a> {
    fn visit_let(&mut self, variable: &'a LValue, expression: &'a Expression) {
        self.output.push_str("LET ");
        self.output.push_str(variable.to_string().as_str());
        self.output.push_str(" = ");
        expression.accept(self);
    }

    fn visit_print(&mut self, content: &'a [Expression]) {
        self.output.push_str("PRINT ");
        for (i, item) in content.iter().enumerate() {
            if i > 0 {
                self.output.push_str("; ");
            }
            item.accept(self);
        }
    }

    fn visit_pause(&mut self, content: &'a [Expression]) {
        self.output.push_str("PAUSE ");
        for (i, item) in content.iter().enumerate() {
            if i > 0 {
                self.output.push_str("; ");
            }
            item.accept(self);
        }
    }

    fn visit_input(&mut self, prompt: Option<&'a Expression>, variable: &'a LValue) {
        self.output.push_str("INPUT ");
        if let Some(prompt) = prompt {
            prompt.accept(self);
            self.output.push_str("; ");
        }
        self.output.push_str(variable.to_string().as_str());
    }

    fn visit_wait(&mut self, time: Option<&'a Expression>) {
        self.output.push_str("WAIT ");
        if let Some(time) = time {
            time.accept(self);
        }
    }

    fn visit_goto(&mut self, line_number: u32) {
        self.output.push_str("GOTO ");
        self.output.push_str(&line_number.to_string());
    }

    fn visit_for(
        &mut self,
        variable: &'a str,
        from: &'a Expression,
        to: &'a Expression,
        step: Option<&'a Expression>,
    ) {
        self.output.push_str("FOR ");
        self.output.push_str(variable);
        self.output.push_str(" = ");
        from.accept(self);
        self.output.push_str(" TO ");
        to.accept(self);
        if let Some(step) = step {
            self.output.push_str(" STEP ");
            step.accept(self);
        }
    }

    fn visit_next(&mut self, variable: &'a str) {
        self.output.push_str("NEXT ");
        self.output.push_str(variable);
    }

    fn visit_end(&mut self) {
        self.output.push_str("END");
    }

    fn visit_gosub(&mut self, line_number: u32) {
        self.output.push_str("GOSUB ");
        self.output.push_str(&line_number.to_string());
    }

    fn visit_return(&mut self) {
        self.output.push_str("RETURN");
    }

    fn visit_if(
        &mut self,
        condition: &'a Expression,
        then: &'a Statement,
        else_: Option<&'a Statement>,
    ) {
        self.output.push_str("IF ");
        condition.accept(self);
        self.output.push_str(" THEN ");
        then.accept(self);
        if let Some(else_) = else_ {
            self.output.push_str(" ELSE ");
            else_.accept(self);
        }
    }

    fn visit_seq(&mut self, statements: &'a [Statement]) {
        for (i, statement) in statements.iter().enumerate() {
            if i > 0 {
                self.output.push_str(": ");
            }
            statement.accept(self);
        }
    }

    fn visit_rem(&mut self, content: &'a str) {
        self.output.push_str(format!("REM {}", content).as_str());
    }

    fn visit_read(&mut self, variables: &'a [LValue]) {
        self.output.push_str("READ ");
        for (i, variable) in variables.iter().enumerate() {
            if i > 0 {
                self.output.push_str(", ");
            }
            self.output.push_str(variable.to_string().as_str());
        }
    }

    fn visit_data(&mut self, values: &'a [DataItem]) {
        self.output.push_str("DATA ");
        for (i, value) in values.iter().enumerate() {
            if i > 0 {
                self.output.push_str(", ");
            }
            match value {
                DataItem::Number(num) => self.output.push_str(&num.to_string()),
                DataItem::String(string) => self.output.push_str(string),
            }
        }
    }

    fn visit_restore(&mut self, line_number: Option<u32>) {
        self.output.push_str("RESTORE ");
        if let Some(line_number) = line_number {
            self.output.push_str(&line_number.to_string());
        }
    }

    fn visit_poke(&mut self, address: u32, values: &'a [u8]) {
        self.output.push_str("POKE ");
        self.output.push_str(&address.to_string());
        self.output.push_str(", ");
        for (i, value) in values.iter().enumerate() {
            if i > 0 {
                self.output.push_str(", ");
            }
            self.output.push_str(&value.to_string());
        }
    }

    fn visit_call(&mut self, address: u32) {
        self.output.push_str("CALL ");
        self.output.push_str(&address.to_string());
    }

    fn visit_dim(&mut self, variable: &'a str, size: u32, length: Option<u32>) {
        self.output.push_str("DIM ");
        self.output.push_str(variable);
        self.output.push('(');
        self.output.push_str(&size.to_string());
        self.output.push(')');

        if let Some(length) = length {
            self.output.push_str(" * ");
            self.output.push_str(&length.to_string());
        }
    }
}

impl<'a> ProgramVisitor<'a> for Printer<'a> {
    fn visit_program(&mut self, program: &'a Program) {
        for (line_number, ast) in program.iter() {
            self.output.push_str(&line_number.to_string());

            ast.accept(self);
            self.output.push('\n');
        }
    }
}
