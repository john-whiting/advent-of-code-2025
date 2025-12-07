use std::ops::RangeInclusive;

use nom::{IResult, bytes::complete::tag, character, sequence::separated_pair, Parser};

fn id_range(input: &str) -> IResult<&str, RangeInclusive<u64>> {
    let (input, (start, end)) =
        separated_pair(character::complete::u64, tag("-"), character::complete::u64)
            .parse(input)?;

    Ok((input, start..=end))
}

fn id_ranges(input: &str) -> IResult<&str, Vec<RangeInclusive<u64>>> {
    nom::multi::separated_list1(tag(","), id_range).parse(input)
}

fn is_repeated(id_str: &str, factor: usize) -> bool {
    if id_str.len() < factor {
        return false;
    }

    let len = id_str.len();
    let chunk_size = if len % factor == 0 {
        len / factor
    } else {
        len / factor + 1
    };

    let chars = id_str.chars().collect::<Vec<char>>();
    let chunks = chars.chunks(chunk_size).collect::<Vec<&[char]>>();

    if let Some((first, rest)) = chunks.split_first() {
        return rest.iter().all(|chunk| chunk == first);
    } else {
        return false;
    }
}

fn part1(input: &str) -> IResult<&str, u64> {
    let (_, ranges) = id_ranges(input)?;
    let sum = ranges.iter()
        .flat_map(|r| r.clone())
        .filter(|v| is_repeated(&v.to_string(), 2))
        .sum();
    Ok((input, sum))
}

fn part2(input: &str) -> IResult<&str, u64> {
    let (_, ranges) = id_ranges(input)?;
    let sum = ranges.iter()
        .flat_map(|r| r.clone())
        .filter(|v| {
            let id_str = v.to_string();
            (2..=id_str.len()).any(|factor| {
                is_repeated(&id_str, factor)
            })
        })
        .sum();
    Ok((input, sum))
}

const INPUT: &str = include_str!("input.txt");

fn main() {
    let (_, result) = part1(INPUT.trim()).unwrap();
    println!("Part 1: {}", result);

    let (_, result) = part2(INPUT.trim()).unwrap();
    println!("Part 2: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_id_range() {
        let input = "100-200";
        let (_, range) = id_range(input).unwrap();
        assert_eq!(range, 100..=200);
    }

    #[test]
    fn test_id_ranges() {
        let input = "100-200,300-400";
        let (_, ranges) = id_ranges(input).unwrap();
        assert_eq!(ranges, vec![100..=200, 300..=400]);
    }

    #[test]
    fn test_is_repeated() {
        assert!(is_repeated("1212", 2));
        assert!(!is_repeated("1234", 2));
        assert!(is_repeated("1188511885", 2));
        assert!(!is_repeated("1188511882", 2));
        assert!(!is_repeated("111", 2));
        assert!(!is_repeated("999", 2));
        assert!(is_repeated("111", 3));
        assert!(is_repeated("999", 3));
    }

    #[test]
    fn test_part1() {
        let (_, count) = part1(EXAMPLE_INPUT).unwrap();
        assert_eq!(count, 1227775554);
    }

    #[test]
    fn test_part2() {
        let (_, count) = part2(EXAMPLE_INPUT).unwrap();
        assert_eq!(count, 4174379265);
    }
}