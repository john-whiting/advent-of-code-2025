use nom::{
    IResult,
    Parser,
};

const INPUT: &str = include_str!("./input.txt");

fn battery(input: &str) -> IResult<&str, u64> {
    let (input, value) = nom::character::complete::one_of("0123456789")(input)?;
    Ok((input, value.to_digit(10).expect("Char to be 0-9") as u64))
}

fn battery_bank(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, batteries) = nom::multi::many1(battery).parse(input)?;
    Ok((input, batteries))
}

fn battery_banks(input: &str) -> IResult<&str, Vec<Vec<u64>>> {
    let (input, banks) = nom::multi::separated_list1(nom::character::complete::newline, battery_bank).parse(input)?;
    Ok((input, banks))
}

fn digits_to_number(digits: &[u64]) -> u64 {
    digits.iter().fold(0, |acc, digit| acc * 10 + digit)
}

fn largest_joltage(bank: &[u64], num_digits: usize) -> u64 {
    let bank_size = bank.len();
    let (_, digits) = (0..num_digits).fold((0, Vec::with_capacity(num_digits)), |(start_index, mut acc), idx| {
        let (found_index, value) = bank[start_index..(bank_size - num_digits + idx + 1)].iter().enumerate().rev().max_by(|(_, a), (_, b)| a.cmp(b)).expect("there should be at least 1 item");

        acc.push(*value);

        return (start_index + found_index + 1, acc);
    });

    // let digits = bank.iter().fold(Vec::with_capacity(num_digits), |mut acc: Vec<u64>, battery| {
    //     if acc.len() >= num_digits {
    //         // Find the index of the largest digit in acc
    //         let (max_index, _) = acc.iter().enumerate().rev().max_by(|a, b| a.1.cmp(b.1)).expect("there is at least one element");

    //         let search_index = if max_index == 0 { num_digits } else { max_index };

    //         // Find the smallest digit in acc to the left of the max value, unless the max is at index 0
    //         let (min_index, _) = acc.iter().take(search_index).enumerate().min_by(|a, b| a.1.cmp(b.1)).expect("there is at least one element");

    //         let current_value = digits_to_number(&acc);
    //         let potential_value = digits_to_number(&{
    //             let mut temp = acc.clone();
    //             temp.remove(min_index);
    //             temp.push(*battery);
    //             temp
    //         });

    //         if current_value >= potential_value {
    //             return acc;
    //         }

    //         acc.remove(min_index);
    //     }

    //     acc.push(*battery);

    //     return acc;
    // });

    return digits_to_number(&digits);
}

fn total_joltage(input: &str, num_digits: usize) -> IResult<&str, u64> {
    let (input, banks) = battery_banks(input)?;

    let total_joltage = banks.iter().map(|bank| largest_joltage(bank, num_digits)).sum();

    return Ok((input, total_joltage));
}

fn main() {
    let (_, result) = total_joltage(INPUT, 2).expect("Parsing to succeed");
    println!("Part 1: {}", result);

    let (_, result) = total_joltage(INPUT, 12).expect("Parsing to succeed");
    println!("Part 2: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"
987654321111111
811111111111119
234234234234278
818181911112111
"#;

    #[test]
    fn test_battery_banks() {
        let (_, banks) = battery_banks(EXAMPLE_INPUT.trim()).unwrap();

        assert_eq!(banks.len(), 4);
        assert_eq!(banks[0].len(), 15);
    }

    #[test]
    fn test_largest_joltage_2() {
        let (_, bank) = battery_bank("987654321111111").unwrap();
        assert_eq!(largest_joltage(&bank, 2), 98);

        let (_, bank) = battery_bank("811111111111119").unwrap();
        assert_eq!(largest_joltage(&bank, 2), 89);

        let (_, bank) = battery_bank("234234234234278").unwrap();
        assert_eq!(largest_joltage(&bank, 2), 78);

        let (_, bank) = battery_bank("818181911112111").unwrap();
        assert_eq!(largest_joltage(&bank, 2), 92);
    }

    #[test]
    fn test_part1_example() {
        let (_, result) = total_joltage(EXAMPLE_INPUT.trim(), 2).unwrap();
        assert_eq!(result, 357);
    }

    #[test]
    fn test_largest_joltage_12() {
        let (_, bank) = battery_bank("987654321111111").unwrap();
        assert_eq!(largest_joltage(&bank, 12), 987654321111);

        let (_, bank) = battery_bank("811111111111119").unwrap();
        assert_eq!(largest_joltage(&bank, 12), 811111111119);

        let (_, bank) = battery_bank("234234234234278").unwrap();
        assert_eq!(largest_joltage(&bank, 12), 434234234278);

        let (_, bank) = battery_bank("818181911112111").unwrap();
        assert_eq!(largest_joltage(&bank, 12), 888911112111);
    }

    #[test]
    fn test_part2_example() {
        let (_, result) = total_joltage(EXAMPLE_INPUT.trim(), 12).unwrap();
        assert_eq!(result, 3121910778619);
    }
}