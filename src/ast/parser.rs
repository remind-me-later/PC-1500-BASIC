use super::{BinaryOperator, Expression, PrintContent, Program, Statement};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{alpha1, alphanumeric0, digit1, multispace0, space1},
    combinator::{map, map_res, opt, recognize},
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    IResult,
};
use std::{cell::RefCell, collections::HashMap, str::FromStr};
use typed_arena::Arena;

pub struct Parser<'parser> {
    stmt_arena: &'parser Arena<Statement<'parser>>,
    expr_arena: &'parser Arena<Expression<'parser>>,
    str_arena: &'parser Arena<String>,

    str_map: RefCell<HashMap<&'parser str, &'parser str>>,
    expr_map: RefCell<HashMap<&'parser Expression<'parser>, &'parser Expression<'parser>>>,
}

impl<'parser> Parser<'parser> {
    pub fn new(
        stmt_arena: &'parser Arena<Statement<'parser>>,
        expr_arena: &'parser Arena<Expression<'parser>>,
        str_arena: &'parser Arena<String>,
    ) -> Self {
        Self {
            stmt_arena,
            expr_arena,
            str_arena,
            str_map: RefCell::new(HashMap::new()),
            expr_map: RefCell::new(HashMap::new()),
        }
    }

    fn get_str(&self, s: &str) -> &'parser str {
        if self.str_map.borrow().contains_key(s) {
            self.str_map.borrow().get(s).unwrap()
        } else {
            let s = self.str_arena.alloc(s.to_string());
            self.str_map.borrow_mut().insert(s, s);
            s
        }
    }

    fn get_expr(&self, expr: Expression<'parser>) -> &'parser Expression<'parser> {
        if self.expr_map.borrow().contains_key(&expr) {
            self.expr_map.borrow().get(&expr).unwrap()
        } else {
            let expr = self.expr_arena.alloc(expr);
            self.expr_map.borrow_mut().insert(expr, expr);
            expr
        }
    }

    fn parse_line_number(input: &str) -> IResult<&str, u32> {
        map_res(digit1, u32::from_str)(input)
    }

    fn parse_number(input: &str) -> IResult<&str, i32> {
        map_res(digit1, i32::from_str)(input)
    }

    // variables are sequences of alphabetic characters, optionally followed by a dollar sign, to indicate a string variable
    fn parse_variable<'input>(
        &'parser self,
        input: &'input str,
    ) -> IResult<&'input str, &'parser str> {
        let (input, name) = recognize(tuple((alpha1, alphanumeric0, opt(tag("$")))))(input)?;
        Ok((input, self.get_str(name)))
    }

    fn parse_factor<'input>(
        &'parser self,
        input: &'input str,
    ) -> IResult<&'input str, &'parser Expression<'parser>> {
        let (input, expr) = alt((
            map(Self::parse_number, |i| {
                self.get_expr(Expression::NumberLiteral(i))
            }),
            map(
                move |i| self.parse_variable(i),
                |i| self.get_expr(Expression::Variable(i)),
            ),
            move |i| self.parse_parens_expression(i),
        ))(input)?;

        Ok((input, expr))
    }

    fn parse_parens_expression<'input>(
        &'parser self,
        input: &'input str,
    ) -> IResult<&'input str, &'parser Expression<'parser>> {
        let (input, expr) = delimited(
            tag("("),
            preceded(multispace0, move |i| {
                let (i, expr) = self.parse_expression(i)?;
                Ok((i, expr))
            }),
            tag(")"),
        )(input)?;

        Ok((input, expr))
    }

    fn parse_mul_div<'input>(
        &'parser self,
        input: &'input str,
    ) -> IResult<&'input str, &'parser Expression<'parser>> {
        fn parse_mul_div_sign(input: &str) -> IResult<&str, BinaryOperator> {
            alt((
                map(tag("*"), |_| BinaryOperator::Mul),
                map(tag("/"), |_| BinaryOperator::Div),
            ))(input)
        }

        let (input, left) = self.parse_factor(input)?;

        // try to parse a multiplication or division operator
        let (input, right) = opt(preceded(multispace0, move |i| {
            let (i, op) = parse_mul_div_sign(i)?;
            let (i, _) = multispace0(i)?;
            let (i, right) = self.parse_mul_div(i)?;

            Ok((i, (op, right)))
        }))(input)?;

        // if we didn't find an operator, return the left expression
        if let Some((op, right)) = right {
            Ok((
                input,
                self.get_expr(Expression::BinaryOp { left, op, right }),
            ))
        } else {
            Ok((input, left))
        }
    }

    fn parse_add_sub<'input>(
        &'parser self,
        input: &'input str,
    ) -> IResult<&'input str, &'parser Expression<'parser>> {
        fn parse_add_sub_sign(input: &str) -> IResult<&str, BinaryOperator> {
            alt((
                map(tag("+"), |_| BinaryOperator::Add),
                map(tag("-"), |_| BinaryOperator::Sub),
            ))(input)
        }

        let (input, left) = self.parse_mul_div(input)?;

        // try to parse an addition or subtraction operator
        let (input, right) = opt(preceded(multispace0, move |i| {
            let (i, op) = parse_add_sub_sign(i)?;
            let (i, _) = multispace0(i)?;
            let (i, right) = self.parse_add_sub(i)?;

            Ok((i, (op, right)))
        }))(input)?;

        // if we didn't find an operator, return the left expression
        if let Some((op, right)) = right {
            Ok((
                input,
                self.get_expr(Expression::BinaryOp { left, op, right }),
            ))
        } else {
            Ok((input, left))
        }
    }

    fn parse_comparison<'input>(
        &'parser self,
        input: &'input str,
    ) -> IResult<&'input str, &'parser Expression<'parser>> {
        fn parse_comparison_sign(input: &str) -> IResult<&str, BinaryOperator> {
            alt((
                map(tag("="), |_| BinaryOperator::Eq),
                map(tag("<>"), |_| BinaryOperator::Ne),
                map(tag("<="), |_| BinaryOperator::Le),
                map(tag(">="), |_| BinaryOperator::Ge),
                map(tag("<"), |_| BinaryOperator::Lt),
                map(tag(">"), |_| BinaryOperator::Gt),
            ))(input)
        }

        let (input, left) = self.parse_add_sub(input)?;

        // try to parse a comparison operator
        let (input, right) = opt(preceded(multispace0, move |i| {
            let (i, op) = parse_comparison_sign(i)?;
            let (i, _) = multispace0(i)?;
            let (i, right) = self.parse_add_sub(i)?;

            Ok((i, (op, right)))
        }))(input)?;

        // if we didn't find an operator, return the left expression
        let var_name = if let Some((op, right)) = right {
            Ok((
                input,
                self.get_expr(Expression::BinaryOp { left, op, right }),
            ))
        } else {
            Ok((input, left))
        };
        var_name
    }

    fn parse_and_or<'input>(
        &'parser self,
        input: &'input str,
    ) -> IResult<&'input str, &'parser Expression<'parser>> {
        fn parse_and_or_sign(input: &str) -> IResult<&str, BinaryOperator> {
            alt((
                map(tag("AND"), |_| BinaryOperator::And),
                map(tag("OR"), |_| BinaryOperator::Or),
            ))(input)
        }

        let (input, left) = self.parse_comparison(input)?;

        // try to parse an AND or OR operator
        let (input, right) = opt(preceded(multispace0, move |i| {
            let (i, op) = parse_and_or_sign(i)?;
            let (i, _) = multispace0(i)?;
            let (i, right) = self.parse_comparison(i)?;

            Ok((i, (op, right)))
        }))(input)?;

        // if we didn't find an operator, return the left expression
        if let Some((op, right)) = right {
            Ok((
                input,
                self.get_expr(Expression::BinaryOp { left, op, right }),
            ))
        } else {
            Ok((input, left))
        }
    }

    fn parse_expression<'input>(
        &'parser self,
        input: &'input str,
    ) -> IResult<&'input str, &'parser Expression<'parser>> {
        self.parse_and_or(input)
    }

    fn parse_let<'input>(
        &'parser self,
        input: &'input str,
    ) -> IResult<&'input str, Statement<'parser>> {
        // LET keyoword is optional
        let (input, _) = opt(tag("LET"))(input)?;
        let (input, _) = multispace0(input)?;
        let (input, variable) = self.parse_variable(input)?;
        let (input, _) = multispace0(input)?;
        let (input, _) = tag("=")(input)?;
        let (input, _) = multispace0(input)?;
        let (input, expression) = self.parse_expression(input)?;

        Ok((
            input,
            Statement::Let {
                variable,
                expression,
            },
        ))
    }

    fn parse_string_literal<'input>(
        &'parser self,
        input: &'input str,
    ) -> IResult<&'input str, &'parser str> {
        let (input, content) =
            delimited(tag("\""), take_while(|c: char| c != '"'), tag("\""))(input)?;

        Ok((input, self.get_str(content)))
    }

    fn parse_print<'input>(
        &'parser self,
        input: &'input str,
    ) -> IResult<&'input str, Statement<'parser>> {
        let (input, _) = tag("PRINT")(input)?;
        let (input, _) = space1(input)?;

        // PRINT can be followed by multiple expressions or string literals separated by semicolons
        let (input, content) = separated_list1(
            // semi-colon followed by optional whitespace
            delimited(tag(";"), multispace0, multispace0),
            alt((
                map(
                    move |i| self.parse_string_literal(i),
                    PrintContent::StringLiteral,
                ),
                map(move |i| self.parse_expression(i), PrintContent::Expression),
            )),
        )(input)?;

        Ok((input, Statement::Print { content }))
    }

    // INPUT "name"; NAME$
    fn parse_input<'input>(
        &'parser self,
        input: &'input str,
    ) -> IResult<&'input str, Statement<'parser>> {
        let (input, _) = tag("INPUT")(input)?;
        let (input, _) = space1(input)?;

        // get optional prompt
        let (input, prompt) = opt(delimited(
            tag("\""),
            take_while(|c: char| c != '"'),
            tag("\""),
        ))(input)?;

        // if a promtp was found, skip the semicolon and optional whitespace
        let (input, _) = opt(delimited(tag(";"), multispace0, multispace0))(input)?;
        let (input, variable) = self.parse_variable(input)?;

        Ok((
            input,
            Statement::Input {
                prompt: prompt.map(|s| self.get_str(s)),
                variable,
            },
        ))
    }

    fn parse_goto<'input>(
        &'parser self,
        input: &'input str,
    ) -> IResult<&'input str, Statement<'parser>> {
        let (input, _) = tag("GOTO")(input)?;
        let (input, _) = space1(input)?;
        let (input, line_number) = Self::parse_line_number(input)?;

        Ok((input, Statement::Goto { line_number }))
    }

    fn parse_gosub<'input>(
        &'parser self,
        input: &'input str,
    ) -> IResult<&'input str, Statement<'parser>> {
        let (input, _) = tag("GOSUB")(input)?;
        let (input, _) = space1(input)?;
        let (input, line_number) = Self::parse_line_number(input)?;

        Ok((input, Statement::GoSub { line_number }))
    }

    fn parse_return<'input>(
        &'parser self,
        input: &'input str,
    ) -> IResult<&'input str, Statement<'parser>> {
        let (input, _) = tag("RETURN")(input)?;

        Ok((input, Statement::Return))
    }

    fn parse_if<'input>(
        &'parser self,
        input: &'input str,
    ) -> IResult<&'input str, Statement<'parser>> {
        let (input, _) = tag("IF")(input)?;
        let (input, _) = multispace0(input)?;
        let (input, condition) = self.parse_expression(input)?;
        let (input, _) = opt(preceded(space1, tag("THEN")))(input)?;
        let (input, _) = space1(input)?;

        let (input, then) = self.parse_atomic_statement(input)?;
        let then = self.stmt_arena.alloc(then);

        let (input, else_) = opt(preceded(space1, move |i| {
            let (i, _) = tag("ELSE")(i)?;
            let (i, _) = space1(i)?;
            let (i, else_) = self.parse_atomic_statement(i)?;
            let else_ = self.stmt_arena.alloc(else_);
            Ok((i, else_))
        }))(input)?;

        Ok((
            input,
            Statement::If {
                condition,
                then,
                else_,
            },
        ))
    }

    fn parse_for<'input>(
        &'parser self,
        input: &'input str,
    ) -> IResult<&'input str, Statement<'parser>> {
        let (input, _) = tag("FOR")(input)?;
        let (input, _) = space1(input)?;
        let (input, variable) = self.parse_variable(input)?;
        let (input, _) = multispace0(input)?;
        let (input, _) = tag("=")(input)?;
        let (input, _) = multispace0(input)?;
        let (input, from) = self.parse_expression(input)?;
        let (input, _) = space1(input)?;
        let (input, _) = tag("TO")(input)?;
        let (input, _) = space1(input)?;
        let (input, to) = self.parse_expression(input)?;
        let (input, step) = opt(preceded(space1, move |i| {
            let (i, _) = tag("STEP")(i)?;
            let (i, _) = space1(i)?;
            let (i, step) = self.parse_expression(i)?;
            Ok((i, step))
        }))(input)?;

        Ok((
            input,
            Statement::For {
                variable,
                from,
                to,
                step,
            },
        ))
    }

    fn parse_next<'input>(
        &'parser self,
        input: &'input str,
    ) -> IResult<&'input str, Statement<'parser>> {
        let (input, _) = tag("NEXT")(input)?;
        let (input, _) = space1(input)?;
        let (input, variable) = self.parse_variable(input)?;

        Ok((input, Statement::Next { variable }))
    }

    fn parse_end<'input>(
        &'parser self,
        input: &'input str,
    ) -> IResult<&'input str, Statement<'parser>> {
        let (input, _) = tag("END")(input)?;

        Ok((input, Statement::End))
    }

    fn parse_atomic_statement<'input>(
        &'parser self,
        input: &'input str,
    ) -> IResult<&'input str, Statement<'parser>> {
        alt((
            move |i| self.parse_let(i),
            move |i| self.parse_print(i),
            move |i| self.parse_input(i),
            move |i| self.parse_goto(i),
            move |i| self.parse_for(i),
            move |i| self.parse_next(i),
            move |i| self.parse_end(i),
            move |i| self.parse_gosub(i),
            move |i| self.parse_if(i),
            move |i| self.parse_return(i),
        ))(input)
    }

    fn parse_statement<'input>(
        &'parser self,
        input: &'input str,
    ) -> IResult<&'input str, Statement<'parser>> {
        let (input, statements) = separated_list1(
            preceded(multispace0, tag(":")),
            preceded(multispace0, move |i| self.parse_atomic_statement(i)),
        )(input)?;

        if statements.len() == 1 {
            let statement = statements.into_iter().next().unwrap();
            Ok((input, statement))
        } else {
            Ok((input, Statement::Seq { statements }))
        }
    }

    // Comment lines start with REM
    fn parse_comment<'input>(&'parser self, input: &'input str) -> IResult<&'input str, ()> {
        let (input, _) = tag("REM")(input)?;
        let (input, _) = take_while(|c: char| c != '\n')(input)?;

        Ok((input, ()))
    }

    fn parse_comment_line<'input>(&'parser self, input: &'input str) -> IResult<&'input str, ()> {
        let (input, _) = tuple((
            move |i| Self::parse_line_number(i),
            space1,
            move |i| self.parse_comment(i),
        ))(input)?;

        Ok((input, ()))
    }

    fn parse_line<'input>(
        &'parser self,
        input: &'input str,
    ) -> IResult<&'input str, (u32, Statement<'parser>)> {
        let (input, (number, _, statement)) = tuple((
            move |i| Self::parse_line_number(i),
            space1,
            move |i| self.parse_statement(i),
        ))(input)?;

        Ok((input, (number, statement)))
    }

    fn parse_program<'input>(
        &'parser self,
        input: &'input str,
    ) -> IResult<&'input str, Program<'parser>> {
        let mut program = Program::new();
        let mut input = input;

        // TODO: improve this loop
        while !input.is_empty() {
            let (new_input, _) = multispace0(input)?;
            if new_input.is_empty() {
                break;
            }

            let (new_input, _) = opt(move |i| self.parse_comment_line(i))(new_input)?;
            let (new_input, _) = multispace0(new_input)?;
            if new_input.is_empty() {
                break;
            }

            let (new_input, line) = self.parse_line(new_input)?;
            program.add_line(line.0, line.1);
            input = new_input;
        }

        Ok((input, program))
    }

    pub fn parse<'input>(
        &'parser self,
        input: &'input str,
    ) -> IResult<&'input str, Program<'parser>> {
        let (input, program) = self.parse_program(input)?;

        Ok((input, program))
    }
}
