use std::ops::RangeInclusive;

use nom::{
    IResult, Parser, bytes::complete::tag, character, multi::many1, sequence::separated_pair,
};

mod utils;

fn fresh_ingredient_range(input: &str) -> IResult<&str, RangeInclusive<u64>> {
    let (input, (start, end)) =
        separated_pair(character::complete::u64, tag("-"), character::complete::u64)
            .parse(input)?;

    Ok((input, start..=end))
}

fn fresh_ingredients(input: &str) -> IResult<&str, Vec<RangeInclusive<u64>>> {
    nom::multi::separated_list1(character::complete::line_ending, fresh_ingredient_range)
        .parse(input)
}

fn ingredient(input: &str) -> IResult<&str, u64> {
    character::complete::u64.parse(input)
}

fn ingredients(input: &str) -> IResult<&str, Vec<u64>> {
    nom::multi::separated_list1(character::complete::line_ending, ingredient).parse(input)
}

struct Inventory {
    fresh_ingredients: Vec<RangeInclusive<u64>>,
    ingredients: Vec<u64>,
}

impl Inventory {
    fn new(fresh_ingredients: Vec<RangeInclusive<u64>>, ingredients: Vec<u64>) -> Self {
        Self {
            fresh_ingredients: utils::reduce_ranges(fresh_ingredients),
            ingredients,
        }
    }

    fn is_fresh(&self, ingredient: u64) -> bool {
        self.fresh_ingredients
            .iter()
            .any(|range| range.contains(&ingredient))
    }

    fn get_fresh_ingredients(&self) -> Vec<u64> {
        self.ingredients
            .iter()
            .cloned()
            .filter(|&ing| self.is_fresh(ing))
            .collect()
    }

    fn max_fresh_ingredient(&self) -> u64 {
        self.fresh_ingredients
            .iter()
            .map(|range| *range.end() - *range.start() + 1)
            .sum()
    }
}

fn inventory(input: &str) -> IResult<&str, Inventory> {
    let (input, (fresh_ingredients, ingredients)) = nom::sequence::separated_pair(
        fresh_ingredients,
        many1(character::complete::line_ending),
        ingredients,
    )
    .parse(input)?;

    Ok((input, Inventory::new(fresh_ingredients, ingredients)))
}

fn part1(input: &str) -> IResult<&str, usize> {
    let (input, inventory) = inventory(input)?;
    Ok((input, inventory.get_fresh_ingredients().len()))
}

fn part2(input: &str) -> IResult<&str, u64> {
    let (input, inventory) = inventory(input)?;
    Ok((input, inventory.max_fresh_ingredient()))
}

const INPUT: &str = include_str!("input.txt");

fn main() {
    let (_, fresh_count) = part1(INPUT).unwrap();
    println!("Part 1: {}", fresh_count);

    let (_, max_fresh) = part2(INPUT).unwrap();
    println!("Part 2: {}", max_fresh);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"
3-5
10-14
16-20
12-18

1
5
8
11
17
32
"#;

    #[test]
    fn test_fresh_ingredient_range() {
        let input = "3-5";
        let (_, range) = fresh_ingredient_range(input).unwrap();
        assert_eq!(range, 3..=5);
    }

    #[test]
    fn test_fresh_ingredients() {
        let input = "3-5\n10-14\n16-20\n12-18";
        let (_, ranges) = fresh_ingredients(input).unwrap();
        assert_eq!(ranges, vec![3..=5, 10..=14, 16..=20, 12..=18]);
    }

    #[test]
    fn test_ingredients() {
        let input = "1\n5\n8\n11\n17\n32";
        let (_, ingredients) = ingredients(input).unwrap();
        assert_eq!(ingredients, vec![1, 5, 8, 11, 17, 32]);
    }

    #[test]
    fn test_inventory() {
        let input = EXAMPLE_INPUT.trim();
        let (_, inventory) = inventory(input).unwrap();
        assert_eq!(
            inventory.fresh_ingredients,
            vec![3..=5, 10..=14, 16..=20, 12..=18]
        );
        assert_eq!(inventory.ingredients, vec![1, 5, 8, 11, 17, 32]);
    }

    #[test]
    fn test_part1() {
        let input = EXAMPLE_INPUT.trim();
        let (_, fresh_count) = part1(input).unwrap();
        assert_eq!(fresh_count, 3); // Ingredients 5, 11, and 17 are fresh
    }

    #[test]
    fn test_part2() {
        let input = EXAMPLE_INPUT.trim();
        let (_, max_fresh) = part2(input).unwrap();
        assert_eq!(max_fresh, 14);
    }
}
