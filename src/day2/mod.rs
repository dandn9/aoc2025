// Note: is there a constant time solution?? This is for sure not optimal
fn solve_part1(lines: Vec<&str>) -> u64 {
    lines[0]
        .split(',')
        .map(|kp| kp.split_once("-"))
        .map(|op| {
            let (a, b) = op.unwrap();
            a.parse::<u64>().unwrap()..=b.parse::<u64>().unwrap()
        })
        .flat_map(|r| r.collect::<Vec<_>>())
        .map(|r| r.to_string())
        .filter(|s| s.len() % 2 == 0)
        .filter(|s| s[..s.len() / 2] == s[s.len() / 2..s.len()])
        .map(|s| s.parse::<u64>().unwrap())
        .sum()
}

fn solve_part2(lines: Vec<&str>) -> u64 {
    lines[0]
        .split(',')
        .map(|kp| kp.split_once("-"))
        .map(|op| {
            let (a, b) = op.unwrap();
            a.parse::<u64>().unwrap()..=b.parse::<u64>().unwrap()
        })
        .flat_map(|r| r.collect::<Vec<_>>())
        .map(|r| r.to_string())
        .filter(|s| {
            let mut slice = &s[..s.len() / 2];
            while slice.len() > 0 {
                if s.len() % slice.len() == 0 && s.split(slice).collect::<String>().is_empty() {
                    return true;
                }
                slice = &slice[..slice.len() - 1];
            }
            return false;
        })
        .map(|s| s.parse::<u64>().unwrap())
        .sum()
}

mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        let file = include_str!("sample.txt");
        let lines: Vec<_> = file.lines().collect();
        assert_eq!(solve_part1(lines), 1227775554);
        let file = include_str!("input.txt");
        let lines: Vec<_> = file.lines().collect();
        assert_eq!(solve_part1(lines), 9188031749);
    }
    #[test]
    fn test_solve_part2() {
        let file = include_str!("sample.txt");
        let lines: Vec<_> = file.lines().collect();
        assert_eq!(solve_part2(lines), 4174379265);
        let file = include_str!("input.txt");
        let lines: Vec<_> = file.lines().collect();
        assert_eq!(solve_part2(lines), 11323661261);
    }
}
