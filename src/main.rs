use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{digit1, multispace0, space1},
    combinator::{map, map_res, opt},
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    IResult,
};
use std::str::FromStr;
use typed_arena::Arena;

enum BinaryOperator {
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    // Logical
    And,
    Or,
    // Comparison
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

impl std::fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Arithmetic
            BinaryOperator::Add => write!(f, "+"),
            BinaryOperator::Sub => write!(f, "-"),
            BinaryOperator::Mul => write!(f, "*"),
            BinaryOperator::Div => write!(f, "/"),
            // Logical
            BinaryOperator::And => write!(f, "AND"),
            BinaryOperator::Or => write!(f, "OR"),
            // Comparison
            BinaryOperator::Eq => write!(f, "="),
            BinaryOperator::Ne => write!(f, "<>"),
            BinaryOperator::Lt => write!(f, "<"),
            BinaryOperator::Le => write!(f, "<="),
            BinaryOperator::Gt => write!(f, ">"),
            BinaryOperator::Ge => write!(f, ">="),
        }
    }
}

enum VariableType {
    Integer,
    String,
}

struct Variable {
    name: String,
    variable_type: VariableType,
}

impl std::fmt::Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.name,
            if let VariableType::String = self.variable_type {
                "$"
            } else {
                ""
            }
        )
    }
}

enum Expression<'a> {
    Literal(i32),
    Variable(Variable),
    BinaryOp(&'a Expression<'a>, BinaryOperator, &'a Expression<'a>),
}

impl std::fmt::Display for Expression<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Literal(value) => write!(f, "{}", value),
            Expression::Variable(variable) => write!(f, "{}", variable),
            Expression::BinaryOp(left, op, right) => {
                write!(f, "({} {} {})", left, op, right)
            }
        }
    }
}

enum PrintContent<'a> {
    Literal(String),
    Expression(Expression<'a>),
}

impl std::fmt::Display for PrintContent<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrintContent::Literal(content) => write!(f, "\"{}\"", content),
            PrintContent::Expression(expr) => write!(f, "{}", expr),
        }
    }
}

enum Statement<'a> {
    Let {
        variable: Variable,
        expression: Expression<'a>,
    },
    Print {
        content: Vec<PrintContent<'a>>,
    },
    Input {
        prompt: Option<String>,
        variable: Variable,
    },
    For {
        variable: Variable,
        from: Expression<'a>,
        to: Expression<'a>,
        step: Option<Expression<'a>>,
    },
    Next {
        variable: Variable,
    },
    Goto {
        line_number: u32,
    },
    End,
    GoSub {
        line_number: u32,
    },
    Return,
    If {
        condition: Expression<'a>,
        then: &'a Statement<'a>,
        else_: Option<&'a Statement<'a>>,
    },
    Seq {
        statements: Vec<Statement<'a>>,
    },
}

impl std::fmt::Display for Statement<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Let {
                variable,
                expression,
            } => {
                write!(f, "LET {} = {}", variable, expression)
            }
            Statement::Print { content } => {
                write!(f, "PRINT ")?;
                for (i, item) in content.iter().enumerate() {
                    if i > 0 {
                        write!(f, "; ")?;
                    }
                    write!(f, "{}", item)?;
                }
                Ok(())
            }
            Statement::Input { prompt, variable } => {
                write!(f, "INPUT ")?;
                if let Some(prompt) = prompt {
                    write!(f, "\"{}\"; ", prompt)?;
                }
                write!(f, "{}", variable)
            }
            Statement::Goto { line_number } => write!(f, "GOTO {}", line_number),
            Statement::For {
                variable,
                from,
                to,
                step,
            } => {
                write!(f, "FOR {} = {} TO {}", variable, from, to)?;
                if let Some(step) = step {
                    write!(f, " STEP {}", step)?;
                }
                Ok(())
            }
            Statement::Next { variable } => write!(f, "NEXT {}", variable),
            Statement::End => write!(f, "END"),
            Statement::GoSub { line_number } => write!(f, "GOSUB {}", line_number),
            Statement::Return => write!(f, "RETURN"),
            Statement::If {
                condition,
                then,
                else_,
            } => {
                write!(f, "IF {} THEN {}", condition, then)?;
                if let Some(else_) = else_ {
                    write!(f, " ELSE {}", else_)?;
                }
                Ok(())
            }
            Statement::Seq { statements } => {
                for (i, statement) in statements.iter().enumerate() {
                    if i > 0 {
                        write!(f, ": ")?;
                    }
                    write!(f, "{}", statement)?;
                }

                Ok(())
            }
        }
    }
}

struct Line<'a> {
    number: u32,
    statement: Statement<'a>,
}

impl std::fmt::Display for Line<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.number, self.statement)
    }
}

struct Program<'a> {
    lines: Vec<Line<'a>>,
}

impl std::fmt::Display for Program<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.lines {
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}

struct Parser<'parser> {
    stmt_arena: Arena<Statement<'parser>>,
    expr_arena: Arena<Expression<'parser>>,
}

impl<'parser> Parser<'parser> {
    pub fn new() -> Self {
        Self {
            stmt_arena: Arena::new(),
            expr_arena: Arena::new(),
        }
    }

    fn parse_line_number<'input>(&'parser self, input: &'input str) -> IResult<&'input str, u32> {
        map_res(digit1, u32::from_str)(input)
    }

    fn parse_number<'input>(&'parser self, input: &'input str) -> IResult<&'input str, i32> {
        map_res(digit1, i32::from_str)(input)
    }

    // variables are sequences of alphabetic characters, optionally followed by a dollar sign, to indicate a string variable
    fn parse_variable<'input>(&'parser self, input: &'input str) -> IResult<&'input str, Variable> {
        let (input, name) = take_while(|c: char| c.is_alphabetic())(input)?;
        let (input, variable_type) = opt(tag("$"))(input)?;

        let variable_type = if variable_type.is_some() {
            VariableType::String
        } else {
            VariableType::Integer
        };

        Ok((
            input,
            Variable {
                name: name.to_string(),
                variable_type,
            },
        ))
    }

    fn parse_factor<'input>(
        &'parser self,
        input: &'input str,
    ) -> IResult<&'input str, Expression<'parser>> {
        alt((
            map(move |i| self.parse_number(i), Expression::Literal),
            map(move |i| self.parse_variable(i), Expression::Variable),
            move |i| self.parse_parens_expression(i),
        ))(input)
    }

    fn parse_parens_expression<'input>(
        &'parser self,
        input: &'input str,
    ) -> IResult<&'input str, Expression<'parser>> {
        delimited(
            tag("("),
            preceded(multispace0, move |i| {
                let (i, expr) = self.parse_expression(i)?;
                Ok((i, expr))
            }),
            tag(")"),
        )(input)
    }

    fn parse_mul_div<'input>(
        &'parser self,
        input: &'input str,
    ) -> IResult<&'input str, Expression<'parser>> {
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
            let left = self.expr_arena.alloc(left);
            let right = self.expr_arena.alloc(right);

            Ok((input, Expression::BinaryOp(left, op, right)))
        } else {
            Ok((input, left))
        }
    }

    fn parse_add_sub<'input>(
        &'parser self,
        input: &'input str,
    ) -> IResult<&'input str, Expression<'parser>> {
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
            let left = self.expr_arena.alloc(left);
            let right = self.expr_arena.alloc(right);

            Ok((input, Expression::BinaryOp(left, op, right)))
        } else {
            Ok((input, left))
        }
    }

    fn parse_comparison<'input>(
        &'parser self,
        input: &'input str,
    ) -> IResult<&'input str, Expression<'parser>> {
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
        if let Some((op, right)) = right {
            let left = self.expr_arena.alloc(left);
            let right = self.expr_arena.alloc(right);

            Ok((input, Expression::BinaryOp(left, op, right)))
        } else {
            Ok((input, left))
        }
    }

    fn parse_and_or<'input>(
        &'parser self,
        input: &'input str,
    ) -> IResult<&'input str, Expression<'parser>> {
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
            let left = self.expr_arena.alloc(left);
            let right = self.expr_arena.alloc(right);

            Ok((input, Expression::BinaryOp(left, op, right)))
        } else {
            Ok((input, left))
        }
    }

    fn parse_expression<'input>(
        &'parser self,
        input: &'input str,
    ) -> IResult<&'input str, Expression<'parser>> {
        self.parse_and_or(input)
    }

    fn parse_let<'input>(
        &'parser self,
        input: &'input str,
    ) -> IResult<&'input str, Statement<'parser>> {
        let (input, _) = tag("LET")(input)?;
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
    ) -> IResult<&'input str, String> {
        let (input, content) =
            delimited(tag("\""), take_while(|c: char| c != '"'), tag("\""))(input)?;
        Ok((input, content.to_string()))
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
                map(move |i| self.parse_string_literal(i), PrintContent::Literal),
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
                prompt: prompt.map(|s| s.to_string()),
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
        let (input, line_number) = self.parse_line_number(input)?;

        Ok((input, Statement::Goto { line_number }))
    }

    fn parse_gosub<'input>(
        &'parser self,
        input: &'input str,
    ) -> IResult<&'input str, Statement<'parser>> {
        let (input, _) = tag("GOSUB")(input)?;
        let (input, _) = space1(input)?;
        let (input, line_number) = self.parse_line_number(input)?;

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
        let then = &*self.stmt_arena.alloc(then);

        let (input, else_) = opt(preceded(space1, move |i| {
            let (i, _) = tag("ELSE")(i)?;
            let (i, _) = space1(i)?;
            let (i, else_) = self.parse_atomic_statement(i)?;
            let else_ = &*self.stmt_arena.alloc(else_);
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
            move |i| self.parse_line_number(i),
            space1,
            move |i| self.parse_comment(i),
        ))(input)?;

        Ok((input, ()))
    }

    fn parse_line<'input>(
        &'parser self,
        input: &'input str,
    ) -> IResult<&'input str, Line<'parser>> {
        let (input, (number, _, statement)) = tuple((
            move |i| self.parse_line_number(i),
            space1,
            move |i| self.parse_statement(i),
        ))(input)?;

        Ok((input, Line { number, statement }))
    }

    fn parse_program<'input>(
        &'parser self,
        input: &'input str,
    ) -> IResult<&'input str, Program<'parser>> {
        let mut lines = Vec::new();
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
            lines.push(line);
            input = new_input;
        }

        // sort lines by line number
        lines.sort_by_key(|line| line.number);

        Ok((input, Program { lines }))
    }
}

fn main() {
    // Read file from first argument
    let input = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();

    let parser = Parser::new();

    match parser.parse_program(&input) {
        Ok((_, program)) => println!("{}", program),
        Err(err) => eprintln!("Error parsing program: {:?}", err),
    }
}
