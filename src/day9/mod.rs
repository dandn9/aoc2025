use std::{
    cmp::{max, min},
    collections::{BTreeMap, HashMap},
    i64,
};

fn solve_part1(lines: Vec<&str>) -> u64 {
    let mut points = lines
        .iter()
        .map(|line| line.split_once(",").unwrap())
        .map(|(x, y)| (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap()))
        .collect::<Vec<(i64, i64)>>();
    points.sort_by_key(|&(x, _)| x);
    (0..points.len())
        .flat_map(|i| {
            (i + 1..points.len())
                .map(|j| {
                    let (x1, y1) = points[i];
                    let (x2, y2) = points[j];
                    // Note: they're sorted by X already
                    (x2 + 1 - x1) as u64 * (max(y1, y2) + 1 - min(y1, y2)) as u64
                })
                .collect::<Vec<_>>()
        })
        .max()
        .unwrap_or(0)
}
fn solve_part2(lines: Vec<&str>) -> u64 {
    let mut points = lines
        .iter()
        .map(|line| line.split_once(",").unwrap())
        .map(|(x, y)| (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap()))
        .collect::<Vec<(i64, i64)>>();
    points.sort_by_key(|&(x, _)| x);
    let mut same_line = HashMap::<(i64, i64), Vec<usize>>::new();
    let mut min_max_y = (i64::MAX, i64::MIN);
    let mut min_max_x = (i64::MAX, i64::MIN);
    points.iter().enumerate().for_each(|(i, &(x, y))| {
        same_line.entry((x, -1)).or_default().push(i);
        same_line.entry((-1, y)).or_default().push(i);
        min_max_y = (min_max_y.0.min(y), min_max_y.1.max(y));
        min_max_x = (min_max_x.0.min(x), min_max_x.1.max(x));
    });
    let mut points_map_x = HashMap::<i64, Vec<i64>>::new();
    let mut points_map_y = HashMap::<i64, Vec<i64>>::new();
    same_line.values().filter(|v| v.len() > 1).for_each(|v| {
        (0..v.len() - 1)
            .map(|i| (points[v[i]], points[v[i + 1]]))
            .for_each(|((x1, y1), (x2, y2))| {
                if x1 == x2 {
                    for v in min(y1, y2)..max(y1, y2) {
                        points_map_y.entry(v).or_default().push(x1);
                    }
                    return;
                }
                let from = min(x1, x2);
                let to = max(x1, x2);

                for v in from..to {
                    points_map_x.entry(v).or_default().push(y1);
                }
            })
    });
    let ranges_map_x = points_map_x
        .iter()
        .map(|(&x, ys)| {
            let mut ys_sorted = ys.clone();
            ys_sorted.sort_unstable();
            let mut ranges = vec![];
            for i in (1..ys_sorted.len()).step_by(2) {
                let from = ys_sorted[i - 1];
                let to = ys_sorted[i];
                ranges.push(from as u64..=to as u64);
            }
            (x, ranges)
        })
        .collect::<HashMap<_, _>>();
    let ranges_map_y = points_map_y
        .iter()
        .map(|(&y, xs)| {
            let mut xs_sorted = xs.clone();
            xs_sorted.sort_unstable();
            let mut ranges = vec![];
            for i in (1..xs_sorted.len()).step_by(2) {
                let from = xs_sorted[i - 1];
                let to = xs_sorted[i];
                ranges.push(from as u64..=to as u64);
            }
            (y, ranges)
        })
        .collect::<HashMap<_, _>>();

    let mut max_area = 0;
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let (x1, y1) = points[i];
            let (x2, y2) = points[j];

            let area = (x2 + 1 - x1) as u64 * (max(y1, y2) + 1 - min(y1, y2)) as u64;

            if area <= max_area {
                continue;
            }

            let y_min = min(y1, y2) as u64;
            let y_max = max(y1, y2) as u64;

            let mut valid = true;
            for x in x1 + 1..x2 {
                let ranges = ranges_map_x.get(&x).unwrap();
                if ranges.iter().all(|r| !r.contains(&y_min)) {
                    valid = false;
                    break;
                }
                if ranges.iter().all(|r| !r.contains(&y_max)) {
                    valid = false;
                    break;
                }
            }
            if !valid {
                continue;
            }

            for y in y_min..y_max {
                let ranges = ranges_map_y.get(&(y as i64)).unwrap();
                if ranges.iter().all(|r| !r.contains(&(x1 as u64))) {
                    valid = false;
                    break;
                }
                if ranges.iter().all(|r| !r.contains(&(x2 as u64))) {
                    valid = false;
                    break;
                }
            }

            if valid {
                max_area = area;
            }
        }
    }

    max_area
}

mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        let file = include_str!("sample.txt");
        let lines: Vec<_> = file.lines().collect();
        assert_eq!(solve_part1(lines), 50);
        let file = include_str!("input.txt");
        let lines: Vec<_> = file.lines().collect();
        assert_eq!(solve_part1(lines), 4764078684);
    }
    #[test]
    fn test_solve_part2() {
        let file = include_str!("sample.txt");
        let lines: Vec<_> = file.lines().collect();
        assert_eq!(solve_part2(lines), 24);
        let file = include_str!("input.txt");
        let lines: Vec<_> = file.lines().collect();
        assert_eq!(solve_part2(lines), 1652344888);
    }
}
