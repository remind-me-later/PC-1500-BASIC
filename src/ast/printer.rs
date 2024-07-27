use std::marker::PhantomData;

use super::{Expression, ExpressionVisitor, Program, ProgramVisitor, Statement, StatementVisitor};

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

    pub fn build(self, ast: &'a Program<'a>) -> String {
        let mut visitor = Printer::new();
        ast.accept(&mut visitor);
        visitor.output
    }

    fn indent(&mut self) {
        self.output.push(' ');
        for _ in 0..self.indent {
            self.output.push(' ');
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

    fn visit_binary_op(
        &mut self,
        left: &Expression<'a>,
        op: super::BinaryOperator,
        right: &Expression<'a>,
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
    fn visit_let(&mut self, variable: &'a str, expression: &Expression<'a>) {
        self.output.push_str("LET ");
        self.output.push_str(variable);
        self.output.push_str(" = ");
        expression.accept(self);
    }

    fn visit_print(&mut self, content: &'a [&'a Expression<'a>]) {
        self.output.push_str("PRINT ");
        for (i, item) in content.iter().enumerate() {
            if i > 0 {
                self.output.push_str("; ");
            }
            item.accept(self);
        }
    }

    fn visit_input(&mut self, prompt: Option<&'a Expression<'a>>, variable: &'a str) {
        self.output.push_str("INPUT ");
        if let Some(prompt) = prompt {
            self.output.push('"');
            prompt.accept(self);
            self.output.push('"');
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
        from: &Expression<'a>,
        to: &Expression<'a>,
        step: Option<&Expression<'a>>,
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
        self.indent += 4;
    }

    fn visit_next(&mut self, variable: &'a str) {
        self.indent -= 4;

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
        condition: &Expression<'a>,
        then: &'a Statement<'a>,
        else_: Option<&'a Statement<'a>>,
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

    fn visit_seq(&mut self, statements: &'a [Statement<'a>]) {
        // colon separated list
        self.output.push('(');
        for (i, statement) in statements.iter().enumerate() {
            if i > 0 {
                self.output.push_str(": ");
            }
            statement.accept(self);
        }
        self.output.push(')');
    }
}

impl<'a> ProgramVisitor<'a> for Printer<'a> {
    fn visit_program(&mut self, program: &'a Program<'a>) {
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
