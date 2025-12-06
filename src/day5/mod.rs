use std::{collections::HashSet, ops::RangeBounds};

fn solve_part1(lines: Vec<&str>) -> u64 {
    let mut lines_iter = lines.iter();
    let ranges = lines_iter
        .by_ref()
        .take_while(|&&line| !line.is_empty())
        .map(|range| {
            range
                .split_once("-")
                .map(|(start, end)| start.parse::<u64>().unwrap()..=end.parse::<u64>().unwrap())
                .unwrap()
        })
        .collect::<Vec<_>>();
    lines_iter
        .filter(|&&line| {
            let num: u64 = line.parse().unwrap();
            ranges.iter().any(|range| range.contains(&num))
        })
        .count() as u64
}
fn solve_part2(lines: Vec<&str>) -> u64 {
    let mut ranges = lines
        .iter()
        .take_while(|&&line| !line.is_empty())
        .map(|range| {
            range
                .split_once("-")
                .map(|(start, end)| start.parse::<u64>().unwrap()..=end.parse::<u64>().unwrap())
                .unwrap()
        })
        .collect::<Vec<_>>();

    while {
        let len_before = ranges.len();
        ranges = ranges.into_iter().fold(Vec::new(), |mut merged, range| {
            if let Some(pos) = merged.iter().position(|r: &std::ops::RangeInclusive<u64>| {
                range.contains(r.start())
                    || range.contains(r.end())
                    || r.contains(range.start())
                    || r.contains(range.end())
            }) {
                let existing = merged.remove(pos);
                merged.push(
                    (*range.start().min(existing.start()))..=(*range.end().max(existing.end())),
                );
            } else {
                merged.push(range);
            }
            merged
        });
        ranges.len() != len_before
    } {}

    ranges.iter().map(|r| r.end() - r.start() + 1).sum()
}

mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        let file = include_str!("sample.txt");
        let lines: Vec<_> = file.lines().collect();
        assert_eq!(solve_part1(lines), 3);
        let file = include_str!("input.txt");
        let lines: Vec<_> = file.lines().collect();
        assert_eq!(solve_part1(lines), 698);
    }
    #[test]
    fn test_solve_part2() {
        let file = include_str!("sample.txt");
        let lines: Vec<_> = file.lines().collect();
        assert_eq!(solve_part2(lines), 14);
        let file = include_str!("input.txt");
        let lines: Vec<_> = file.lines().collect();
        assert_eq!(solve_part2(lines), 352807801032167);
    }
}
