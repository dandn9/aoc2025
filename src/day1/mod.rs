fn solve_part1(lines: Vec<&str>) -> u32 {
    lines
        .iter()
        .map(|line| {
            if line.as_bytes()[0] == b'L' {
                -line[1..].parse::<i32>().unwrap()
            } else {
                line[1..].parse::<i32>().unwrap()
            }
        })
        .fold((50, 0 as u32), |acc, x| {
            let r = (acc.0 + x + 100) % 100;
            (r, acc.1 + if r == 0 { 1 } else { 0 })
        })
        .1
}
fn solve_part2(lines: Vec<&str>) -> u32 {
    lines
        .iter()
        .map(|line| {
            if line.as_bytes()[0] == b'L' {
                -line[1..].parse::<i32>().unwrap()
            } else {
                line[1..].parse::<i32>().unwrap()
            }
        })
        .fold((50, 0 as u32), |acc, x| {
            let spins = (x.abs() / 100) as u32;
            let r = (acc.0 + x).rem_euclid(100);
            let changed_sign = match (x, r) {
                (..0, r) if r > acc.0 && acc.0 != 0 => 1,
                (1.., r) if r < acc.0 && acc.0 != 0 => 1,
                (_, 0) => 1,
                _ => 0,
            };
            (r, acc.1 + spins + changed_sign)
        })
        .1
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
        assert_eq!(solve_part1(lines), 1145);
    }
    #[test]
    fn test_solve_part2() {
        let file = include_str!("sample.txt");
        let lines: Vec<_> = file.lines().collect();
        assert_eq!(solve_part2(lines), 6);
        let file = include_str!("input.txt");
        let lines: Vec<_> = file.lines().collect();
        assert_eq!(solve_part2(lines), 6561);
    }
}
