use super::{BinaryOperator, Expression, Program, Statement};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{alpha1, alphanumeric0, digit1, multispace0, multispace1, space1},
    combinator::{map, map_res, opt, recognize},
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    IResult,
};
use std::str::FromStr;

pub fn parse(input: &str) -> IResult<&str, Program> {
    let (input, program) = parse_program(input)?;

    Ok((input, program))
}

fn parse_line_number(input: &str) -> IResult<&str, u32> {
    map_res(digit1, u32::from_str)(input)
}

fn parse_number(input: &str) -> IResult<&str, i32> {
    map_res(digit1, i32::from_str)(input)
}

fn parse_string_literal(input: &str) -> IResult<&str, String> {
    let (input, content) = delimited(tag("\""), take_while(|c: char| c != '"'), tag("\""))(input)?;

    Ok((input, content.to_string()))
}

// variables are sequences of alphabetic characters, optionally followed by a dollar sign, to indicate a string variable
fn parse_variable(input: &str) -> IResult<&str, String> {
    let (input, name) = recognize(tuple((alpha1, alphanumeric0, opt(tag("$")))))(input)?;
    Ok((input, name.to_string()))
}

fn parse_factor(input: &str) -> IResult<&str, Expression> {
    let (input, expr) = alt((
        map(parse_number, Expression::NumberLiteral),
        map(parse_variable, Expression::Variable),
        map(parse_string_literal, Expression::StringLiteral),
        parse_parens_expression,
    ))(input)?;

    Ok((input, expr))
}

fn parse_parens_expression(input: &str) -> IResult<&str, Expression> {
    let (input, expr) = delimited(
        tag("("),
        delimited(multispace0, parse_expression, multispace0),
        tag(")"),
    )(input)?;

    Ok((input, expr))
}

fn parse_mul_div(input: &str) -> IResult<&str, Expression> {
    fn parse_mul_div_sign(input: &str) -> IResult<&str, BinaryOperator> {
        alt((
            map(tag("*"), |_| BinaryOperator::Mul),
            map(tag("/"), |_| BinaryOperator::Div),
        ))(input)
    }

    let (input, left) = parse_factor(input)?;

    // try to parse a multiplication or division operator
    let (input, right) = opt(move |i| {
        let (i, _) = multispace0(i)?;
        let (i, op) = parse_mul_div_sign(i)?;
        let (i, _) = multispace0(i)?;
        let (i, right) = parse_mul_div(i)?;

        Ok((i, (op, right)))
    })(input)?;

    // if we didn't find an operator, return the left expression
    if let Some((op, right)) = right {
        let left = Box::new(left);
        let right = Box::new(right);
        Ok((input, Expression::Binary { left, op, right }))
    } else {
        Ok((input, left))
    }
}

fn parse_add_sub(input: &str) -> IResult<&str, Expression> {
    fn parse_add_sub_sign(input: &str) -> IResult<&str, BinaryOperator> {
        alt((
            map(tag("+"), |_| BinaryOperator::Add),
            map(tag("-"), |_| BinaryOperator::Sub),
        ))(input)
    }

    let (input, left) = parse_mul_div(input)?;

    // try to parse an addition or subtraction operator
    let (input, right) = opt(move |i| {
        let (i, _) = multispace0(i)?;
        let (i, op) = parse_add_sub_sign(i)?;
        let (i, _) = multispace0(i)?;
        let (i, right) = parse_add_sub(i)?;

        Ok((i, (op, right)))
    })(input)?;

    // if we didn't find an operator, return the left expression
    if let Some((op, right)) = right {
        let left = Box::new(left);
        let right = Box::new(right);
        Ok((input, Expression::Binary { left, op, right }))
    } else {
        Ok((input, left))
    }
}

fn parse_comparison(input: &str) -> IResult<&str, Expression> {
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

    let (input, left) = parse_add_sub(input)?;

    // try to parse a comparison operator
    let (input, right) = opt(move |i| {
        let (i, _) = multispace0(i)?;
        let (i, op) = parse_comparison_sign(i)?;
        let (i, _) = multispace0(i)?;
        let (i, right) = parse_add_sub(i)?;

        Ok((i, (op, right)))
    })(input)?;

    // if we didn't find an operator, return the left expression
    if let Some((op, right)) = right {
        let left = Box::new(left);
        let right = Box::new(right);
        Ok((input, Expression::Binary { left, op, right }))
    } else {
        Ok((input, left))
    }
}

fn parse_and(input: &str) -> IResult<&str, Expression> {
    let (input, left) = parse_comparison(input)?;

    // try to parse an AND operator
    let (input, right) = opt(move |i| {
        let (i, _) = multispace0(i)?;
        let (i, _) = tag("AND")(i)?;
        let (i, _) = multispace0(i)?;
        let (i, right) = parse_comparison(i)?;

        Ok((i, right))
    })(input)?;

    // if we didn't find an operator, return the left expression
    if let Some(right) = right {
        let left = Box::new(left);
        let right = Box::new(right);
        Ok((
            input,
            Expression::Binary {
                left,
                op: BinaryOperator::And,
                right,
            },
        ))
    } else {
        Ok((input, left))
    }
}

fn parse_or(input: &str) -> IResult<&str, Expression> {
    let (input, left) = parse_and(input)?;

    // try to parse an OR operator
    let (input, right) = opt(move |i| {
        let (i, _) = multispace0(i)?;
        let (i, _) = tag("OR")(i)?;
        let (i, _) = multispace0(i)?;
        let (i, right) = parse_and(i)?;

        Ok((i, right))
    })(input)?;

    // if we didn't find an operator, return the left expression
    if let Some(right) = right {
        let left = Box::new(left);
        let right = Box::new(right);
        Ok((
            input,
            Expression::Binary {
                left,
                op: BinaryOperator::Or,
                right,
            },
        ))
    } else {
        Ok((input, left))
    }
}

fn parse_expression(input: &str) -> IResult<&str, Expression> {
    parse_or(input)
}

fn parse_let(input: &str) -> IResult<&str, Statement> {
    // LET keyoword is optional
    let (input, _) = opt(tag("LET"))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, variable) = parse_variable(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, expression) = parse_expression(input)?;

    Ok((
        input,
        Statement::Let {
            variable,
            expression,
        },
    ))
}

fn parse_print(input: &str) -> IResult<&str, Statement> {
    let (input, _) = tag("PRINT")(input)?;
    let (input, _) = space1(input)?;

    // PRINT can be followed by multiple expressions or string literals separated by semicolons
    let (input, content) = separated_list1(
        // semi-colon followed by optional whitespace
        delimited(tag(";"), multispace0, multispace0),
        move |i| parse_expression(i),
    )(input)?;

    Ok((input, Statement::Print { content }))
}

// INPUT "name"; NAME$
fn parse_input(input: &str) -> IResult<&str, Statement> {
    let (input, _) = tag("INPUT")(input)?;
    let (input, _) = space1(input)?;

    // get optional prompt
    let (input, prompt) = opt(move |i| parse_expression(i))(input)?;

    // if a promtp was found, skip the semicolon and optional whitespace
    let (input, _) = opt(delimited(tag(";"), multispace0, multispace0))(input)?;
    let (input, variable) = parse_variable(input)?;

    Ok((input, Statement::Input { prompt, variable }))
}

fn parse_goto(input: &str) -> IResult<&str, Statement> {
    let (input, _) = tag("GOTO")(input)?;
    let (input, _) = space1(input)?;
    let (input, line_number) = parse_line_number(input)?;

    Ok((input, Statement::Goto { line_number }))
}

fn parse_gosub(input: &str) -> IResult<&str, Statement> {
    let (input, _) = tag("GOSUB")(input)?;
    let (input, _) = space1(input)?;
    let (input, line_number) = parse_line_number(input)?;

    Ok((input, Statement::GoSub { line_number }))
}

fn parse_return(input: &str) -> IResult<&str, Statement> {
    let (input, _) = tag("RETURN")(input)?;

    Ok((input, Statement::Return))
}

fn parse_if(input: &str) -> IResult<&str, Statement> {
    let (input, _) = tag("IF")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, condition) = parse_expression(input)?;
    let (input, _) = opt(preceded(space1, tag("THEN")))(input)?;
    let (input, _) = space1(input)?;

    let (input, then) = parse_statement(input)?;
    let then = Box::new(then);

    let (input, else_) = opt(preceded(space1, move |i| {
        let (i, _) = tag("ELSE")(i)?;
        let (i, _) = space1(i)?;
        let (i, else_) = parse_statement(i)?;
        let else_ = Box::new(else_);
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

fn parse_for(input: &str) -> IResult<&str, Statement> {
    let (input, _) = tag("FOR")(input)?;
    let (input, _) = space1(input)?;
    let (input, variable) = parse_variable(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, from) = parse_expression(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = tag("TO")(input)?;
    let (input, _) = space1(input)?;
    let (input, to) = parse_expression(input)?;
    let (input, step) = opt(preceded(space1, move |i| {
        let (i, _) = tag("STEP")(i)?;
        let (i, _) = space1(i)?;
        let (i, step) = parse_expression(i)?;
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

fn parse_next(input: &str) -> IResult<&str, Statement> {
    let (input, _) = tag("NEXT")(input)?;
    let (input, _) = space1(input)?;
    let (input, variable) = parse_variable(input)?;

    Ok((input, Statement::Next { variable }))
}

fn parse_end(input: &str) -> IResult<&str, Statement> {
    map(tag("END"), |_| Statement::End)(input)
}

fn parse_atomic_statement(input: &str) -> IResult<&str, Statement> {
    alt((
        parse_let,
        parse_print,
        parse_input,
        parse_goto,
        parse_for,
        parse_next,
        parse_end,
        parse_gosub,
        parse_if,
        parse_return,
    ))(input)
}

fn parse_statement(input: &str) -> IResult<&str, Statement> {
    let (input, statements) = separated_list1(
        preceded(multispace0, tag(":")),
        preceded(multispace0, parse_atomic_statement),
    )(input)?;

    if statements.len() == 1 {
        let statement = statements.into_iter().next().unwrap();
        Ok((input, statement))
    } else {
        Ok((input, Statement::Seq { statements }))
    }
}

// Comment lines start with REM
fn parse_comment(input: &str) -> IResult<&str, ()> {
    let (input, _) = tag("REM")(input)?;
    let (input, _) = take_while(|c: char| c != '\n')(input)?;

    Ok((input, ()))
}

fn parse_comment_line(input: &str) -> IResult<&str, ()> {
    let (input, _) = tuple((parse_line_number, multispace1, parse_comment))(input)?;

    Ok((input, ()))
}

fn parse_line(input: &str) -> IResult<&str, (u32, Statement)> {
    let (input, (number, _, statement)) =
        tuple((parse_line_number, multispace1, parse_statement))(input)?;

    Ok((input, (number, statement)))
}

fn parse_program(input: &str) -> IResult<&str, Program> {
    let mut program = Program::new();
    let mut input = input;

    // TODO: improve this loop
    while !input.is_empty() {
        let (new_input, _) = multispace0(input)?;
        if new_input.is_empty() {
            break;
        }

        let (new_input, _) = opt(parse_comment_line)(new_input)?;
        let (new_input, _) = multispace0(new_input)?;
        if new_input.is_empty() {
            break;
        }

        let (new_input, (line_number, statement)) = parse_line(new_input)?;
        program.add_line(line_number, statement);
        input = new_input;
    }

    Ok((input, program))
}
