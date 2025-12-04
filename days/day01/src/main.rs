use nom::{
    IResult,
    Parser,
};

const INPUT: &str = include_str!("./input.txt");

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Left(i64),
    Right(i64),
}

fn direction(input: &str) -> IResult<&str, Direction> {
    let (input, dir) = nom::character::complete::one_of("LR")(input)?;
    let (input, value) = nom::character::complete::i64(input)?;

    match dir {
        'L' => Ok((input, Direction::Left(value))),
        'R' => Ok((input, Direction::Right(value))),
        _ => unreachable!(),
    }
}

fn directions(input: &str) -> IResult<&str, Vec<Direction>> {
    nom::multi::separated_list0(nom::character::complete::line_ending, direction).parse(input)
}

fn step(position: i64, dir: &Direction) -> (i64, i64) {
    match dir {
        Direction::Left(steps) => {
            let new_position = position - steps;
            let zeros_crossed = match position {
                _ if new_position > 0 => 0,
                p if p == 0 => new_position / -100,
                _ => (new_position / -100) + 1,
            };
            (new_position.rem_euclid(100), zeros_crossed)
        }
        Direction::Right(steps) => {
            let new_position = position + steps;
            let zeros_crossed = new_position / 100;
            (new_position.rem_euclid(100), zeros_crossed)
        }
    }
}


fn part1(input: &str) -> IResult<&str, i64> {
    let (input, dirs) = directions(input)?;

    let (_, counted_zeros) = dirs.iter().fold((50, 0), |(position, counted_zeros), dir| {
        let (new_position, _) = step(position, dir);
        let new_counted_zeros = if new_position == 0 {
            counted_zeros + 1
        } else {
            counted_zeros
        };
        (new_position, new_counted_zeros)
    });

    return Ok((input, counted_zeros));
}

fn part2(input: &str) -> IResult<&str, i64> {
    let (input, dirs) = directions(input)?;

    let (_, counted_zeros) = dirs.iter().fold((50, 0), |(position, counted_zeros), dir| {
        let (new_position, zeros_crossed) = step(position, dir);
        (new_position, counted_zeros + zeros_crossed)
    });

    return Ok((input, counted_zeros));
}


fn main() {
    let (_, result) = part1(INPUT.trim()).unwrap();
    println!("Day 01: {}", result);
    
    let (_, result) = part2(INPUT.trim()).unwrap();
    println!("Day 02: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"#;

    #[test]
    fn test_directions() {
        let (_, dirs) = directions(EXAMPLE_INPUT.trim()).unwrap();
        assert_eq!(dirs.len(), 10);
        assert_eq!(dirs[0], Direction::Left(68));
        assert_eq!(dirs[1], Direction::Left(30));
        assert_eq!(dirs[2], Direction::Right(48));
    }

    #[test]
    fn test_part1_example() {
        let (_, result) = part1(EXAMPLE_INPUT.trim()).unwrap();
        assert_eq!(result, 3);
    }

    #[test]
    fn test_day02_example() {
        let (_, result) = part2(EXAMPLE_INPUT.trim()).unwrap();
        assert_eq!(result, 6);
    }

    #[test]
    fn test_part1() {
        let (_, result) = part1(INPUT.trim()).unwrap();
        assert_eq!(result, 1076);
    }

    #[test]
    fn test_day02() {
        let (_, result) = part2(INPUT.trim()).unwrap();
        assert_eq!(result, 6379);
    }

    #[test]
    fn test_step() {
        let (pos, zeros) = step(50, &Direction::Left(68));
        assert_eq!(pos, 82);
        assert_eq!(zeros, 1);

        let (pos, zeros) = step(pos, &Direction::Left(30));
        assert_eq!(pos, 52);
        assert_eq!(zeros, 0);

        let (pos, zeros) = step(pos, &Direction::Right(48));
        assert_eq!(pos, 0);
        assert_eq!(zeros, 1);

        let (pos, zeros) = step(pos, &Direction::Left(5));
        assert_eq!(pos, 95);
        assert_eq!(zeros, 0);

        let (pos, zeros) = step(pos, &Direction::Right(60));
        assert_eq!(pos, 55);
        assert_eq!(zeros, 1);

        let (pos, zeros) = step(pos, &Direction::Left(55));
        assert_eq!(pos, 0);
        assert_eq!(zeros, 1);

        let (pos, zeros) = step(pos, &Direction::Left(1));
        assert_eq!(pos, 99);
        assert_eq!(zeros, 0);

        let (pos, zeros) = step(pos, &Direction::Left(99));
        assert_eq!(pos, 0);
        assert_eq!(zeros, 1);

        let (pos, zeros) = step(pos, &Direction::Right(14));
        assert_eq!(pos, 14);
        assert_eq!(zeros, 0);

        let (pos, zeros) = step(pos, &Direction::Left(82));
        assert_eq!(pos, 32);
        assert_eq!(zeros, 1);
    }

    #[test]
    fn test_input() {
    }
}
