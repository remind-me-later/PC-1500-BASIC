use super::{BinaryOperator, Expression, Program, Statement};

pub trait ExpressionVisitor<'a, RetTy = ()> {
    fn visit_number_literal(&mut self, num: i32) -> RetTy;
    fn visit_string_literal(&mut self, content: &'a str) -> RetTy;
    fn visit_variable(&mut self, variable: &'a str) -> RetTy;
    fn visit_binary_op(
        &mut self,
        left: &'a Expression,
        op: BinaryOperator,
        right: &'a Expression,
    ) -> RetTy;
}

impl<'a> Expression {
    pub fn accept<V: ExpressionVisitor<'a, RetTy>, RetTy>(&'a self, visitor: &mut V) -> RetTy {
        match self {
            Expression::NumberLiteral(num) => visitor.visit_number_literal(*num),
            Expression::StringLiteral(content) => visitor.visit_string_literal(content),
            Expression::Variable(variable) => visitor.visit_variable(variable),
            Expression::Binary { left, op, right } => visitor.visit_binary_op(left, *op, right),
        }
    }
}

pub trait StatementVisitor<'a, RetTy = ()> {
    fn visit_let(&mut self, variable: &'a str, expression: &'a Expression) -> RetTy;
    fn visit_print(&mut self, content: &'a [Expression]) -> RetTy;
    fn visit_input(&mut self, prompt: Option<&'a Expression>, variable: &'a str) -> RetTy;
    fn visit_goto(&mut self, line_number: u32) -> RetTy;
    fn visit_for(
        &mut self,
        variable: &'a str,
        from: &'a Expression,
        to: &'a Expression,
        step: Option<&'a Expression>,
    ) -> RetTy;
    fn visit_next(&mut self, variable: &'a str) -> RetTy;
    fn visit_end(&mut self) -> RetTy;
    fn visit_gosub(&mut self, line_number: u32) -> RetTy;
    fn visit_return(&mut self) -> RetTy;
    fn visit_if(
        &mut self,
        condition: &'a Expression,
        then: &'a Statement,
        else_: Option<&'a Statement>,
    ) -> RetTy;
    fn visit_seq(&mut self, statements: &'a [Statement]) -> RetTy;
    fn visit_rem(&mut self, content: &'a str) -> RetTy;
}

impl<'a> Statement {
    pub fn accept<V: StatementVisitor<'a, RetTy>, RetTy>(&'a self, visitor: &mut V) -> RetTy {
        match self {
            Statement::Let {
                variable,
                expression,
            } => visitor.visit_let(variable, expression),
            Statement::Print { content } => visitor.visit_print(content.as_slice()),
            Statement::Input { prompt, variable } => visitor.visit_input(prompt.as_ref(), variable),
            Statement::Goto { line_number } => visitor.visit_goto(*line_number),
            Statement::For {
                variable,
                from,
                to,
                step,
            } => visitor.visit_for(variable, from, to, step.as_ref()),
            Statement::Next { variable } => visitor.visit_next(variable),
            Statement::End => visitor.visit_end(),
            Statement::GoSub { line_number } => visitor.visit_gosub(*line_number),
            Statement::Return => visitor.visit_return(),
            Statement::If {
                condition,
                then,
                else_,
            } => visitor.visit_if(condition, then, else_.as_deref()),
            Statement::Seq { statements } => visitor.visit_seq(statements),
            Statement::Rem { content } => visitor.visit_rem(content),
        }
    }
}

pub trait ProgramVisitor<'a, RetTy = ()> {
    fn visit_program(&mut self, program: &'a Program) -> RetTy;
}

impl<'a> Program {
    pub fn accept<V: ProgramVisitor<'a, RetTy>, RetTy>(&'a self, visitor: &mut V) -> RetTy {
        visitor.visit_program(self)
    }
}
