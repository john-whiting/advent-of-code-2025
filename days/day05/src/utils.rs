use std::{cmp, ops::RangeInclusive};

pub fn reduce_ranges(mut ranges: Vec<RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
    ranges.sort_by_key(|range| *range.start());

    ranges.into_iter()
        .fold(Vec::new(), |mut acc: Vec<RangeInclusive<u64>>, range| {
            if let Some(last_range) = acc.last_mut() {
                if *range.start() <= (*last_range.end() + 1) {
                    let new_end = cmp::max(*last_range.end(), *range.end());
                    *last_range = *last_range.start()..=new_end;
                    return acc
                }
            }
            acc.push(range);
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_irreducible() {
        let mut ranges = vec![1..=3, 5..=7, 9..=11];
        ranges = reduce_ranges(ranges);
        assert_eq!(ranges, vec![1..=3, 5..=7, 9..=11]);
    }

    #[test]
    fn test_partial_overlap() {
        let mut ranges = vec![1..=5, 4..=8, 10..=12];
        ranges = reduce_ranges(ranges);
        assert_eq!(ranges, vec![1..=8, 10..=12]);
    }

    #[test]
    fn test_full_overlap() {
        let mut ranges = vec![1..=10, 2..=5, 6..=9];
        ranges = reduce_ranges(ranges);
        assert_eq!(ranges, vec![1..=10]);
    }

    #[test]
    fn test_adjacent_ranges() {
        let mut ranges = vec![1..=3, 4..=6, 7..=9];
        ranges = reduce_ranges(ranges);
        assert_eq!(ranges, vec![1..=9]);
    }

    #[test]
    fn test_mixed_ranges() {
        let mut ranges = vec![1..=4, 3..=5, 7..=9, 8..=10, 12..=15];
        ranges = reduce_ranges(ranges);
        assert_eq!(ranges, vec![1..=5, 7..=10, 12..=15]);
    }

    #[test]
    fn test_empty_input() {
        let mut ranges: Vec<RangeInclusive<u64>> = vec![];
        ranges = reduce_ranges(ranges);
        assert_eq!(ranges, vec![]);
    }

    #[test]
    fn test_single_range() {
        let mut ranges = vec![1..=5];
        ranges = reduce_ranges(ranges);
        assert_eq!(ranges, vec![1..=5]);
    }

    #[test]
    fn test_unsorted_input() {
        let mut ranges = vec![5..=7, 1..=3, 4..=6];
        ranges = reduce_ranges(ranges);
        assert_eq!(ranges, vec![1..=7]);
    }
}