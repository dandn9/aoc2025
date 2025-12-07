fn solve_part1(lines: Vec<&str>) -> u64 {
    let mut nums: Vec<u64> = vec![0; lines.len() - 1];

    let mut left = 0;
    let mut res = 0;

    for i in 0..=lines[0].len() {
        let mut is_column = true;
        for k in 0..lines.len() {
            if i < lines[0].len() && !lines[k].chars().nth(i).unwrap().is_whitespace() {
                is_column = false;
            }
        }

        if is_column {
            for k in 0..lines.len() - 1 {
                nums[k] = lines[k][left..i].trim().parse::<u64>().unwrap();
            }
            let op = lines.last().unwrap()[left..i].trim();
            res += match op {
                "*" => nums.iter().product::<u64>() as u64,
                "+" => nums.iter().sum::<u64>() as u64,
                _ => panic!("Unknown op"),
            };

            left = i + 1;
        }
    }
    res
}
fn solve_part2(lines: Vec<&str>) -> u64 {
    let mut right = lines[0].len();
    let mut res = 0;

    let mut nums_strs = Vec::<String>::new();
    for i in (0..lines[0].len()).rev() {
        let str = (0..lines.len() - 1)
            .map(|k| lines[k].chars().nth(i).unwrap())
            .collect::<String>()
            .trim()
            .to_owned();

        if !str.is_empty() {
            nums_strs.push(str.clone());
        }

        if str.is_empty() || i == 0 {
            let nums = nums_strs
                .iter()
                .map(|s| s.trim().parse::<u64>().unwrap())
                .collect::<Vec<u64>>();

            nums_strs.clear();
            let op = lines.last().unwrap()[i..right].trim();
            res += match op {
                "*" => nums.iter().product::<u64>() as u64,
                "+" => nums.iter().sum::<u64>() as u64,
                _ => panic!("Unknown op"),
            };

            right = i;
        }
    }
    res
}

mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        let file = include_str!("sample.txt");
        let lines: Vec<_> = file.lines().collect();
        assert_eq!(solve_part1(lines), 4277556);
        let file = include_str!("input.txt");
        let lines: Vec<_> = file.lines().collect();
        assert_eq!(solve_part1(lines), 4412382293768);
    }
    #[test]
    fn test_solve_part2() {
        let file = include_str!("sample.txt");
        let lines: Vec<_> = file.lines().collect();
        assert_eq!(solve_part2(lines), 3263827);
        let file = include_str!("input.txt");
        let lines: Vec<_> = file.lines().collect();
        assert_eq!(solve_part2(lines), 7858808482092);
    }
}
