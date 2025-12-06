use nom::{
    IResult, Parser,
    character::{
        self,
        complete::{multispace0, newline, one_of, space0},
    },
    combinator::opt,
    error::ParseError,
    multi::many1,
    sequence::{delimited, terminated},
};

mod utils;

const INPUT: &str = include_str!("./input.txt");

fn number(input: &str) -> IResult<&str, u64> {
    delimited(space0, character::complete::u64, space0).parse(input)
}

fn numbers(input: &str) -> IResult<&str, Vec<u64>> {
    many1(terminated(number, opt(newline))).parse(input)
}

fn number_row(input: &str) -> IResult<&str, Vec<u64>> {
    many1(number).parse(input)
}

fn number_rows(input: &str) -> IResult<&str, Vec<Vec<u64>>> {
    many1(terminated(number_row, opt(newline))).parse(input)
}

#[derive(PartialEq, Debug)]
enum Operator {
    MULT,
    ADD,
}

impl Operator {
    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Operator::MULT => a * b,
            Operator::ADD => a + b,
        }
    }
}

fn operator(input: &str) -> IResult<&str, Operator> {
    let (input, char) = delimited(space0, one_of("*+"), space0).parse(input)?;

    let operator = match char {
        '*' => Operator::MULT,
        '+' => Operator::ADD,
        _ => unreachable!(),
    };

    Ok((input, operator))
}

fn operator_list(input: &str) -> IResult<&str, Vec<Operator>> {
    many1(operator).parse(input)
}

struct Expression {
    operator: Operator,
    operands: Vec<u64>,
}

fn expression(input: &str) -> IResult<&str, Expression> {
    let (input, operands) = numbers(input)?;
    let (input, operator) = operator(input)?;

    Ok((input, Expression { operator, operands }))
}

fn expressions(input: &str) -> IResult<&str, Vec<Expression>> {
    many1(terminated(expression, multispace0)).parse(input)
}

#[derive(PartialEq, Debug)]
struct ParsedInput {
    number_rows: Vec<Vec<u64>>,
    operators: Vec<Operator>,
}

fn parsed_input(input: &str) -> IResult<&str, ParsedInput> {
    let (input, number_rows) = number_rows(input)?;
    let (input, operators) = operator_list(input)?;

    Ok((
        input,
        ParsedInput {
            number_rows,
            operators,
        },
    ))
}

fn part1(input: &str) -> IResult<&str, u64> {
    let (input, parsed) = parsed_input(input)?;

    Ok((
        input,
        (0..parsed.operators.len())
            .map(|idx| {
                let operator = &parsed.operators[idx];

                parsed
                    .number_rows
                    .iter()
                    .map(|number_row| number_row[idx])
                    .reduce(|a, b| operator.apply(a, b))
                    .expect("there should be at least one number row")
            })
            .sum(),
    ))
}

#[derive(Debug, PartialEq)]
pub struct InvalidExpressions {
    pub rotated_input: String,
}

impl<'a> ParseError<&'a str> for InvalidExpressions {
    fn from_error_kind(_input: &str, _kind: nom::error::ErrorKind) -> Self {
        InvalidExpressions {
            rotated_input: String::new(),
        }
    }

    fn append(_input: &str, _kind: nom::error::ErrorKind, other: Self) -> Self {
        other
    }
}

fn part2(input: &str) -> IResult<&str, u64, InvalidExpressions> {
    let rotated = &utils::rotate_anticlockwise(input);
    let (_, exprs) = expressions(rotated)
        .map_err(|e| e.map(|er| InvalidExpressions::from_error_kind(er.input, er.code)))?;

    let result = exprs
        .into_iter()
        .map(|expr| {
            expr.operands
                .into_iter()
                .reduce(|a, b| expr.operator.apply(a, b))
                .expect("there should be at least one operand")
        })
        .sum();

    Ok(("", result))
}

fn main() {
    let (_, result) = part1(INPUT).unwrap();
    println!("Part 1 result: {}", result);

    let (_, result) = part2(INPUT).unwrap();
    println!("Part 2 result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "#;

    #[test]
    fn test_number_row() {
        let (_, row) = number_row("123 328  51 64 ").unwrap();
        assert_eq!(row, vec![123, 328, 51, 64]);

        let (_, row) = number_row(" 45 64  387 23 ").unwrap();
        assert_eq!(row, vec![45, 64, 387, 23]);
    }

    #[test]
    fn test_number_rows() {
        let (_, rows) = number_rows("123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n").unwrap();
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0], vec![123, 328, 51, 64]);
        assert_eq!(rows[1], vec![45, 64, 387, 23]);
        assert_eq!(rows[2], vec![6, 98, 215, 314]);
    }

    #[test]
    fn test_parsed_input() {
        let (_, parsed) = parsed_input(EXAMPLE_INPUT.trim()).unwrap();

        assert_eq!(parsed.number_rows.len(), 3);
        assert_eq!(parsed.operators.len(), 4);

        assert_eq!(
            parsed,
            ParsedInput {
                number_rows: vec![
                    vec![123, 328, 51, 64],
                    vec![45, 64, 387, 23],
                    vec![6, 98, 215, 314],
                ],
                operators: vec![Operator::MULT, Operator::ADD, Operator::MULT, Operator::ADD,],
            }
        )
    }

    #[test]
    fn test_part1() {
        let (_, result) = part1(EXAMPLE_INPUT.trim()).unwrap();
        assert_eq!(result, 4277556);
    }

    #[test]
    fn test_expression() {
        let (_, expr) = expression("123\n456\n789*").unwrap();
        assert_eq!(expr.operator, Operator::MULT);
        assert_eq!(expr.operands, vec![123, 456, 789]);
    }

    #[test]
    fn test_expressions() {
        let example_input = utils::rotate_anticlockwise(EXAMPLE_INPUT.trim_start());
        let (input, exprs) = expressions(&example_input).unwrap();
        dbg!(input);
        assert_eq!(exprs.len(), 4);

        assert_eq!(exprs[0].operator, Operator::ADD);
        assert_eq!(exprs[0].operands, vec![4, 431, 623]);
    }

    #[test]
    fn test_part2() {
        let (_, result) = part2(EXAMPLE_INPUT.trim_start()).unwrap();
        assert_eq!(result, 3263827);
    }
}
