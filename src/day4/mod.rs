fn solve_part1(lines: Vec<&str>) -> u64 {
    lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(x, c)| match c {
                    '@' => Some((x, y)),
                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .filter(|&(x, y)| {
            let neighbors = DIRS
                .iter()
                .map(|(dx, dy)| (dx + x as i32, dy + y as i32))
                .filter(|&(x, y)| {
                    y >= 0
                        && x >= 0
                        && lines
                            .get(y as usize)
                            .and_then(|line| line.chars().nth(x as usize))
                            == Some('@')
                })
                .count();
            neighbors <= 3
        })
        .count() as u64
}

fn solve_part1_mut(lines: &mut Vec<Vec<char>>) -> u64 {
    let coords = lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter_map(|(x, c)| match c {
                    '@' => Some((x, y)),
                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .filter(|&(x, y)| {
            let neighbors = DIRS
                .iter()
                .map(|(dx, dy)| (dx + x as i32, dy + y as i32))
                .filter(|&(x, y)| {
                    y >= 0
                        && x >= 0
                        && lines.get(y as usize).and_then(|line| line.get(x as usize)) == Some(&'@') // Note: this works since Option<&char> implements PartialEq, we're not comparing references here.
                })
                .count();
            neighbors <= 3
        })
        .collect::<Vec<_>>();

    coords.iter().for_each(|&(x, y)| lines[y][x] = '.');
    coords.len() as u64
}

fn solve_part2(lines: Vec<&str>) -> u64 {
    let mut chars_vec = lines
        .iter()
        .map(|str| str.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut acc = 0;
    loop {
        let count = solve_part1_mut(&mut chars_vec);
        if count == 0 {
            break;
        }
        acc += count;
    }
    acc
}

static DIRS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, 1),
    (0, -1),
    (1, 1),
    (1, -1),
    (1, 0),
];
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        let file = include_str!("sample.txt");
        let lines: Vec<_> = file.lines().collect();
        assert_eq!(solve_part1(lines), 13);
        let file = include_str!("input.txt");
        let lines: Vec<_> = file.lines().collect();
        assert_eq!(solve_part1(lines), 1370);
    }

    #[test]
    fn test_solve_part2() {
        let file = include_str!("sample.txt");
        let lines: Vec<_> = file.lines().collect();
        assert_eq!(solve_part2(lines), 43);
        let file = include_str!("input.txt");
        let lines: Vec<_> = file.lines().collect();
        assert_eq!(solve_part2(lines), 8437);
    }
}
