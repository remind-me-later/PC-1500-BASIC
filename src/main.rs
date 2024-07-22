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

#[derive(Debug)]
enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
}

impl std::fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOperator::Add => write!(f, "+"),
            BinaryOperator::Sub => write!(f, "-"),
            BinaryOperator::Mul => write!(f, "*"),
            BinaryOperator::Div => write!(f, "/"),
        }
    }
}

#[derive(Debug)]
enum VariableType {
    Integer,
    String,
}

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
enum PrintContent<'a> {
    Literal(String),
    Expression(Expression<'a>),
}

impl std::fmt::Display for PrintContent<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrintContent::Literal(content) => write!(f, "{}", content),
            PrintContent::Expression(expr) => write!(f, "{}", expr),
        }
    }
}

#[derive(Debug)]
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
    Goto(u32),
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
            Statement::Goto(line_number) => write!(f, "GOTO {}", line_number),
        }
    }
}

#[derive(Debug)]
struct Line<'a> {
    number: u32,
    statement: Statement<'a>,
}

impl std::fmt::Display for Line<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.number, self.statement)
    }
}

#[derive(Debug)]
struct Program<'a> {
    lines: Vec<&'a Line<'a>>,
}

impl std::fmt::Display for Program<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.lines {
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}

struct Parser<'a> {
    arena: Arena<Line<'a>>,
    expr_arena: Arena<Expression<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new() -> Self {
        Self {
            arena: Arena::new(),
            expr_arena: Arena::new(),
        }
    }

    fn parse_line_number<'b>(&'a self, input: &'b str) -> IResult<&'b str, u32> {
        map_res(digit1, u32::from_str)(input)
    }

    fn parse_number<'b>(&'a self, input: &'b str) -> IResult<&'b str, i32> {
        map_res(digit1, i32::from_str)(input)
    }

    // variables are sequences of alphabetic characters, optionally followed by a dollar sign, to indicate a string variable
    fn parse_variable<'b>(&'a self, input: &'b str) -> IResult<&'b str, Variable> {
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

    fn parse_factor<'b>(&'a self, input: &'b str) -> IResult<&'b str, Expression<'a>> {
        alt((
            map(move |i| self.parse_number(i), Expression::Literal),
            map(move |i| self.parse_variable(i), Expression::Variable),
            move |i| self.parse_parens_expression(i),
        ))(input)
    }

    fn parse_parens_expression<'b>(&'a self, input: &'b str) -> IResult<&'b str, Expression<'a>> {
        delimited(
            tag("("),
            preceded(multispace0, move |i| {
                let (i, expr) = self.parse_expression(i)?;
                Ok((i, expr))
            }),
            tag(")"),
        )(input)
    }

    fn parse_mul_div<'b>(&'a self, input: &'b str) -> IResult<&'b str, Expression<'a>> {
        fn parse_mul_div_sign(input: &str) -> IResult<&str, BinaryOperator> {
            alt((
                map(tag("*"), |_| BinaryOperator::Mul),
                map(tag("/"), |_| BinaryOperator::Div),
            ))(input)
        }

        let (input, left) = self.parse_factor(input)?;
        let (input, _) = multispace0(input)?;

        // try to parse a multiplication or division operator
        // if we didn't find an operator, return the left expression
        if let Ok((input, op)) = parse_mul_div_sign(input) {
            let (input, _) = multispace0(input)?;
            let (input, right) = self.parse_mul_div(input)?;

            let left = self.expr_arena.alloc(left);
            let right = self.expr_arena.alloc(right);

            Ok((input, Expression::BinaryOp(left, op, right)))
        } else {
            Ok((input, left))
        }
    }

    fn parse_add_sub<'b>(&'a self, input: &'b str) -> IResult<&'b str, Expression<'a>> {
        fn parse_add_sub_sign(input: &str) -> IResult<&str, BinaryOperator> {
            alt((
                map(tag("+"), |_| BinaryOperator::Add),
                map(tag("-"), |_| BinaryOperator::Sub),
            ))(input)
        }

        let (input, left) = self.parse_mul_div(input)?;
        let (input, _) = multispace0(input)?;

        // try to parse an addition or subtraction operator
        // if we didn't find an operator, return the left expression
        if let Ok((input, op)) = parse_add_sub_sign(input) {
            let (input, _) = multispace0(input)?;
            let (input, right) = self.parse_add_sub(input)?;

            let left = self.expr_arena.alloc(left);
            let right = self.expr_arena.alloc(right);

            Ok((input, Expression::BinaryOp(left, op, right)))
        } else {
            Ok((input, left))
        }
    }

    // TODO: is this the best way to handle the recursive nature of the parser?
    fn parse_expression<'b>(&'a self, input: &'b str) -> IResult<&'b str, Expression<'a>> {
        self.parse_add_sub(input)
    }

    fn parse_let<'b>(&'a self, input: &'b str) -> IResult<&'b str, Statement<'a>> {
        let (input, _) = tag("LET")(input)?;
        let (input, _) = space1(input)?;
        let (input, variable) = self.parse_variable(input)?;
        let (input, _) = space1(input)?;
        let (input, _) = tag("=")(input)?;
        let (input, _) = space1(input)?;
        let (input, expression) = self.parse_expression(input)?;

        Ok((
            input,
            Statement::Let {
                variable,
                expression,
            },
        ))
    }

    fn parse_string_literal<'b>(&'a self, input: &'b str) -> IResult<&'b str, String> {
        let (input, content) =
            delimited(tag("\""), take_while(|c: char| c != '"'), tag("\""))(input)?;
        Ok((input, content.to_string()))
    }

    fn parse_print<'b>(&'a self, input: &'b str) -> IResult<&'b str, Statement<'a>> {
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
    fn parse_input<'b>(&'a self, input: &'b str) -> IResult<&'b str, Statement<'a>> {
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

    fn parse_goto<'b>(&'a self, input: &'b str) -> IResult<&'b str, Statement<'a>> {
        let (input, _) = tag("GOTO")(input)?;
        let (input, _) = space1(input)?;
        let (input, line_number) = self.parse_line_number(input)?;

        Ok((input, Statement::Goto(line_number)))
    }

    fn parse_statement<'b>(&'a self, input: &'b str) -> IResult<&'b str, Statement<'a>> {
        alt((
            move |i| self.parse_let(i),
            move |i| self.parse_print(i),
            move |i| self.parse_input(i),
            move |i| self.parse_goto(i),
        ))(input)
    }

    fn parse_line<'b>(&'a self, input: &'b str) -> IResult<&'b str, &'a Line<'a>> {
        let (input, (number, _, statement)) = tuple((
            move |i| self.parse_line_number(i),
            space1,
            move |i| self.parse_statement(i),
        ))(input)?;
        let line = self.arena.alloc(Line { number, statement });
        Ok((input, line))
    }

    fn parse_program<'b>(&'a self, input: &'b str) -> IResult<&'b str, Program<'a>> {
        let mut lines = Vec::new();
        let mut input = input;

        // TODO: improve this loop
        while !input.is_empty() {
            let (new_input, _) = multispace0(input)?;
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
    let input = r#"
5 LET X = 1 * Y + 1
10 PRINT "Hello, World!"; X
15 INPUT "What is your name?"; NAME$
20 GOTO 10
"#;

    let parser = Parser::new();

    match parser.parse_program(input) {
        Ok((_, program)) => println!("{}", program),
        Err(err) => eprintln!("Error parsing program: {:?}", err),
    }
}
