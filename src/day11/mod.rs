use std::{cell::RefCell, collections::HashMap, rc::Rc};

fn traverse(
    map: &HashMap<&str, Vec<&str>>,
    node: &str,
    target: &str,
    cache: Rc<RefCell<HashMap<String, u64>>>,
) -> u64 {
    if !cache.borrow().contains_key(node) {
        if node == target {
            cache.borrow_mut().insert(node.to_string(), 1);
        } else {
            let empty_vec = vec![];
            let list = map.get(node).unwrap_or(&empty_vec);
            let res = (0..list.len())
                .map(|i| traverse(map, list[i], target, cache.clone()))
                .sum::<u64>();
            cache.borrow_mut().insert(node.to_string(), res);
        }
    }

    *cache.borrow().get(node).unwrap()
}

fn solve_part1(lines: Vec<&str>) -> u64 {
    traverse(
        &lines
            .iter()
            .map(|line| {
                (
                    line.split_once(":").unwrap().0,
                    line.split_once(":")
                        .unwrap()
                        .1
                        .trim()
                        .split(" ")
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<HashMap<_, _>>(),
        "you",
        "out",
        Rc::new(RefCell::new(HashMap::new())),
    )
}

fn solve_part2(lines: Vec<&str>) -> u64 {
    let map = &lines
        .iter()
        .map(|line| {
            (
                line.split_once(":").unwrap().0,
                line.split_once(":")
                    .unwrap()
                    .1
                    .trim()
                    .split(" ")
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<HashMap<_, _>>();

    traverse(&map, "svr", "fft", Rc::new(RefCell::new(HashMap::new())))
        * traverse(&map, "fft", "dac", Rc::new(RefCell::new(HashMap::new())))
        * traverse(&map, "dac", "out", Rc::new(RefCell::new(HashMap::new())))
}
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        let file = include_str!("sample.txt");
        let lines: Vec<_> = file.lines().collect();
        assert_eq!(solve_part1(lines), 5);
        let file = include_str!("input.txt");
        let lines: Vec<_> = file.lines().collect();
        assert_eq!(solve_part1(lines), 494);
    }
    #[test]
    fn test_solve_part2() {
        let file = include_str!("sample2.txt");
        let lines: Vec<_> = file.lines().collect();
        assert_eq!(solve_part2(lines), 2);
        let file = include_str!("input.txt");
        let lines: Vec<_> = file.lines().collect();
        assert_eq!(solve_part2(lines), 296006754704850);
    }
}
