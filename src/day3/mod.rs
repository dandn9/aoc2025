fn solve_part1(lines: Vec<&str>) -> u64 {
    lines
        .iter()
        .map(|line| {
            let num = line
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .fold((0, 0), |acc, c| {
                    if acc.1 > acc.0 {
                        (acc.1, c)
                    } else {
                        if c > acc.1 { (acc.0, c) } else { acc }
                    }
                });

            format!("{}{}", num.0, num.1).parse::<u64>().unwrap()
        })
        .sum()
}
fn solve_part2(lines: Vec<&str>) -> u64 {
    lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .fold([0; 12], |mut acc, c| {
                    if (0..=10).any(|i| acc[i] < acc[i + 1]) {
                        let i = (0..=10).find(|&i| acc[i] < acc[i + 1]).unwrap();
                        (i..=10).for_each(|i| acc[i] = acc[i + 1]);
                        acc[11] = c;
                        acc
                    } else {
                        if c > acc[11] {
                            acc[11] = c
                        }
                        acc
                    }
                })
                .iter()
                .map(|&d| d.to_string())
                .collect::<String>()
                .parse::<u64>()
                .unwrap()
        })
        .sum()
}

mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        let file = include_str!("sample.txt");
        let lines: Vec<_> = file.lines().collect();
        assert_eq!(solve_part1(lines), 357);
        let file = include_str!("input.txt");
        let lines: Vec<_> = file.lines().collect();
        assert_eq!(solve_part1(lines), 357);
    }

    #[test]
    fn test_solve_part2() {
        let file = include_str!("sample.txt");
        let lines: Vec<_> = file.lines().collect();
        assert_eq!(solve_part2(lines), 3121910778619);
        let file = include_str!("input.txt");
        let lines: Vec<_> = file.lines().collect();
        assert_eq!(solve_part2(lines), 3121910778619);
    }
}
