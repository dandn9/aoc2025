use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};
fn get_splitters(lines: &Vec<&str>) -> HashSet<(usize, usize)> {
    lines
        .iter()
        .enumerate()
        .filter_map(|(y, line)| {
            Some(
                line.char_indices()
                    .filter_map(|(x, c)| if c == '^' { Some(x) } else { None })
                    .map(|x| (x, y))
                    .collect::<Vec<_>>(),
            )
        })
        .flat_map(|f| f)
        .collect::<HashSet<(usize, usize)>>()
}

fn solve_part1(lines: Vec<&str>) -> u64 {
    let mut beams = lines
        .iter()
        .enumerate()
        .filter_map(|(y, line)| {
            if let Some(x) = line.chars().position(|c| c == 'S') {
                Some((x, y))
            } else {
                None
            }
        })
        .collect::<HashSet<(usize, usize)>>();
    let splitters = get_splitters(&lines);

    let mut split = 0;
    (0..lines.len()).for_each(|_| {
        beams = beams
            .iter()
            .flat_map(|pos| {
                let new_pos = (pos.0, pos.1 + 1);
                if splitters.contains(&new_pos) {
                    split += 1;
                    vec![(new_pos.0 - 1, new_pos.1), (new_pos.0 + 1, new_pos.1)]
                } else {
                    vec![new_pos]
                }
            })
            .collect::<HashSet<(usize, usize)>>();
    });
    split
}

fn solve_part2(lines: Vec<&str>) -> u64 {
    let mut beams = lines
        .iter()
        .enumerate()
        .filter_map(|(y, line)| {
            if let Some(x) = line.chars().position(|c| c == 'S') {
                Some(((x, y), 1))
            } else {
                None
            }
        })
        .collect::<HashMap<(usize, usize), usize>>();
    let splitters = get_splitters(&lines);

    (0..lines.len()).for_each(|_| {
        let new_beams = beams
            .iter()
            .flat_map(|(&(x, y), count)| {
                let new_pos = (x, y + 1);
                if splitters.contains(&new_pos) {
                    vec![
                        (new_pos.0 - 1, new_pos.1, *count),
                        (new_pos.0 + 1, new_pos.1, *count),
                    ]
                } else {
                    vec![(new_pos.0, new_pos.1, *count)]
                }
            })
            .collect::<Vec<(usize, usize, usize)>>();

        let mut beams_map = new_beams
            .iter()
            .map(|&(x, y, _)| ((x, y), 0))
            .collect::<HashMap<(usize, usize), usize>>();

        new_beams.iter().for_each(|&(x, y, count)| {
            *beams_map.get_mut(&(x, y)).unwrap() += count;
        });

        beams = beams_map;
    });
    beams.iter().map(|(_, &count)| count).sum::<usize>() as u64
}
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        let file = include_str!("sample.txt");
        let lines: Vec<_> = file.lines().collect();
        assert_eq!(solve_part1(lines), 21);
        let file = include_str!("input.txt");
        let lines: Vec<_> = file.lines().collect();
        assert_eq!(solve_part1(lines), 1562);
    }
    #[test]
    fn test_solve_part2() {
        let file = include_str!("sample.txt");
        let lines: Vec<_> = file.lines().collect();
        assert_eq!(solve_part2(lines), 40);
        let file = include_str!("input.txt");
        let lines: Vec<_> = file.lines().collect();
        assert_eq!(solve_part2(lines), 24292631346665);
    }
}
