use std::marker::PhantomData;

use super::{
    node::UnaryOperator, Expression, ExpressionVisitor, Program, ProgramVisitor, Statement,
    StatementVisitor,
};

pub struct Printer<'a> {
    indent: usize,
    output: String,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> Printer<'a> {
    pub fn new() -> Self {
        Printer {
            indent: 0,
            output: String::new(),
            _phantom: PhantomData,
        }
    }

    pub fn build(self, ast: &'a Program) -> String {
        let mut visitor = Printer::new();
        ast.accept(&mut visitor);
        visitor.output
    }

    fn indent(&mut self) {
        self.output.push('\t');
        for _ in 0..self.indent {
            self.output.push('\t');
        }
    }
}

impl<'a> ExpressionVisitor<'a> for Printer<'a> {
    fn visit_number_literal(&mut self, num: i32) {
        self.output.push_str(&num.to_string());
    }

    fn visit_variable(&mut self, variable: &'a str) {
        self.output.push_str(variable);
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
    fn visit_let(&mut self, variable: &'a str, expression: &'a Expression) {
        self.output.push_str("LET ");
        self.output.push_str(variable);
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

    fn visit_input(&mut self, prompt: Option<&'a Expression>, variable: &'a str) {
        self.output.push_str("INPUT ");
        if let Some(prompt) = prompt {
            prompt.accept(self);
            self.output.push_str("; ");
        }
        self.output.push_str(variable);
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
        self.indent += 1;
    }

    fn visit_next(&mut self, variable: &'a str) {
        self.indent -= 1;

        // TODO: should be ok
        if self.output.ends_with('\t') {
            self.output.pop();
        }

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
}

impl<'a> ProgramVisitor<'a> for Printer<'a> {
    fn visit_program(&mut self, program: &'a Program) {
        for (line_number, ast) in program.iter() {
            // print line number and then indent
            // 10 LET a = 1
            // 20 FOR i = 1 TO 10
            // 30     PRINT i
            // 40     NEXT i
            self.output.push_str(&line_number.to_string());
            self.indent();

            ast.accept(self);
            self.output.push('\n');
        }
    }
}
