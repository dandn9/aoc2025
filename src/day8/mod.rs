use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    hash::Hash,
};
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Vec3 {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}
impl Vec3 {
    fn dist(a: &Vec3, b: &Vec3) -> f32 {
        let dx = (a.x - b.x) as f32;
        let dy = (a.y - b.y) as f32;
        let dz = (a.z - b.z) as f32;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

#[derive(Clone, Copy, Debug)]
struct DistEntry {
    from: Vec3,
    to: Vec3,
    distance: f32,
}

impl PartialEq for DistEntry {
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}
impl Eq for DistEntry {}
impl PartialOrd for DistEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DistEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.total_cmp(&other.distance)
    }
}

fn solve_part1(lines: Vec<&str>, iterations: i32, top: i32) -> u64 {
    let vecs = lines
        .iter()
        .map(|line| {
            let nums = line.split(",").collect::<Vec<_>>();
            Vec3 {
                x: nums[0].parse().unwrap(),
                y: nums[1].parse().unwrap(),
                z: nums[2].parse().unwrap(),
            }
        })
        .collect::<Vec<_>>();
    let combinations = (0..vecs.len())
        .flat_map(|parent| {
            (parent + 1..vecs.len())
                .map(|child| DistEntry {
                    from: vecs[parent],
                    to: vecs[child],
                    distance: -Vec3::dist(&vecs[parent], &vecs[child]),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // This is min_heap since we negate the distance
    let mut min_heap = BinaryHeap::<DistEntry>::from(combinations);

    let mut clusters: Vec<Vec<Vec3>> = vec![];
    let mut node_to_cluster_index: HashMap<Vec3, usize> = HashMap::new();
    let mut i = 0;

    while let Some(combination) = min_heap.pop() {
        if i >= iterations {
            break;
        }
        i += 1;
        /*
         * Cases:
         * 1. both parent and child do not have a cluster: form a cluster
         * 2. One of each is not assigned to a cluster and the other is: join the missing node
         * 3. both are assigned to different clusters: join the clusters
         */

        if !node_to_cluster_index.contains_key(&combination.from)
            && !node_to_cluster_index.contains_key(&combination.to)
        {
            // Case 1
            let index = clusters.len();
            clusters.push(vec![combination.from, combination.to]);
            node_to_cluster_index.insert(combination.from, index);
            node_to_cluster_index.insert(combination.to, index);
        } else if !node_to_cluster_index.contains_key(&combination.from)
            && node_to_cluster_index.contains_key(&combination.to)
        {
            // Case 2
            let cluster_index = node_to_cluster_index.get(&combination.to).cloned().unwrap();
            node_to_cluster_index.insert(combination.from, cluster_index);
            clusters[cluster_index].push(combination.from);
        } else if node_to_cluster_index.contains_key(&combination.from)
            && !node_to_cluster_index.contains_key(&combination.to)
        {
            // Case 2
            let cluster_index = node_to_cluster_index
                .get(&combination.from)
                .cloned()
                .unwrap();
            node_to_cluster_index.insert(combination.to, cluster_index);
            clusters[cluster_index].push(combination.to);
        } else {
            // Case 3
            let from_cluster = node_to_cluster_index
                .get(&combination.from)
                .cloned()
                .unwrap();
            let to_cluster = node_to_cluster_index.get(&combination.to).cloned().unwrap();

            if from_cluster == to_cluster {
                continue;
            }

            // Leave an empty cluster, so that we dont have to move every "next" cluster in the array
            let target = from_cluster.min(to_cluster);
            let moved = from_cluster.max(to_cluster);
            let moved_cluster = clusters[moved].clone();

            for node in &moved_cluster {
                node_to_cluster_index.insert(node.clone(), target);
            }

            let target_cluster = clusters[target].clone();
            clusters[target] = [target_cluster, moved_cluster].concat();
            clusters[moved] = vec![];
        }
    }

    let mut cluster_sizes = clusters
        .iter_mut()
        .filter_map(|k| if k.len() == 0 { None } else { Some(k.len()) })
        .collect::<Vec<_>>();

    cluster_sizes.sort();

    cluster_sizes
        .iter()
        .rev()
        .take(top as usize)
        .product::<usize>() as u64
}

fn solve_part2(lines: Vec<&str>) -> u64 {
    let mut vecs = lines
        .iter()
        .map(|line| {
            let nums = line.split(",").collect::<Vec<_>>();
            Vec3 {
                x: nums[0].parse().unwrap(),
                y: nums[1].parse().unwrap(),
                z: nums[2].parse().unwrap(),
            }
        })
        .collect::<Vec<_>>();
    let combinations = (0..vecs.len())
        .flat_map(|parent| {
            (parent + 1..vecs.len())
                .map(|child| DistEntry {
                    from: vecs[parent],
                    to: vecs[child],
                    distance: -Vec3::dist(&vecs[parent], &vecs[child]),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // This is min_heap since we negate the distance
    let mut min_heap = BinaryHeap::<DistEntry>::from(combinations);

    let mut clusters: Vec<Vec<Vec3>> = vec![];
    let mut node_to_cluster_index: HashMap<Vec3, usize> = HashMap::new();

    while let Some(combination) = min_heap.pop() {
        /*
         * Cases:
         * 1. both parent and child do not have a cluster: form a cluster
         * 2. One of each is not assigned to a cluster and the other is: join the missing node
         * 3. both are assigned to different clusters: join the clusters
         */

        if !node_to_cluster_index.contains_key(&combination.from)
            && !node_to_cluster_index.contains_key(&combination.to)
        {
            // Case 1
            let index = clusters.len();
            clusters.push(vec![combination.from, combination.to]);
            node_to_cluster_index.insert(combination.from, index);
            node_to_cluster_index.insert(combination.to, index);
        } else if !node_to_cluster_index.contains_key(&combination.from)
            && node_to_cluster_index.contains_key(&combination.to)
        {
            // Case 2
            let cluster_index = node_to_cluster_index.get(&combination.to).cloned().unwrap();
            node_to_cluster_index.insert(combination.from, cluster_index);
            clusters[cluster_index].push(combination.from);
        } else if node_to_cluster_index.contains_key(&combination.from)
            && !node_to_cluster_index.contains_key(&combination.to)
        {
            // Case 2
            let cluster_index = node_to_cluster_index
                .get(&combination.from)
                .cloned()
                .unwrap();
            node_to_cluster_index.insert(combination.to, cluster_index);
            clusters[cluster_index].push(combination.to);
        } else {
            // Case 3
            let from_cluster = node_to_cluster_index
                .get(&combination.from)
                .cloned()
                .unwrap();
            let to_cluster = node_to_cluster_index.get(&combination.to).cloned().unwrap();

            if from_cluster == to_cluster {
                continue;
            }

            // Leave an empty cluster, so that we dont have to move every "next" cluster in the array
            let target = from_cluster.min(to_cluster);
            let moved = from_cluster.max(to_cluster);
            let moved_cluster = clusters[moved].clone();

            for node in &moved_cluster {
                node_to_cluster_index.insert(node.clone(), target);
            }

            let target_cluster = clusters[target].clone();
            clusters[target] = [target_cluster, moved_cluster].concat();
            clusters[moved] = vec![];
        }
        if let Some(from_index) = vecs.iter().position(|x| *x == combination.from) {
            vecs.remove(from_index);
        }
        if let Some(to_index) = vecs.iter().position(|x| *x == combination.to) {
            vecs.remove(to_index);
        }

        if vecs.len() > 0 {
            continue;
        }

        let mut non_empty_cluster: i32 = 0;
        for cluster in &clusters {
            if cluster.len() > 0 {
                non_empty_cluster += 1;
            }
        }
        if non_empty_cluster == 1 {
            println!(
                "Non-empty clusters: {} - {:?}",
                non_empty_cluster, combination
            );
            return combination.from.x as u64 * combination.to.x as u64;
        }
    }
    panic!("No solution found");
}

mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        let file = include_str!("sample.txt");
        let lines: Vec<_> = file.lines().collect();
        assert_eq!(solve_part1(lines, 10, 3), 40);
        let file = include_str!("input.txt");
        let lines: Vec<_> = file.lines().collect();
        assert_eq!(solve_part1(lines, 1000, 3), 175440);
    }

    #[test]
    fn test_solve_part2() {
        let file = include_str!("sample.txt");
        let lines: Vec<_> = file.lines().collect();
        assert_eq!(solve_part2(lines), 25272);
        let file = include_str!("input.txt");
        let lines: Vec<_> = file.lines().collect();
        assert_eq!(solve_part2(lines), 3200955921);
    }
}
