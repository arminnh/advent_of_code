use std::{collections::HashMap, mem::swap};

type Box3D = [u32; 3];

// Connect 1000 pairs of nearest junction boxes. What do you get after multiplying together the sizes of the three largest circuits
pub fn part_1(input: &str) -> u64 {
    part_1_for_n_pairs(input, 1000)
}

fn part_1_for_n_pairs(input: &str, n: usize) -> u64 {
    let boxes = parse_input(input);
    let mut distances = calculate_distances(&boxes);
    distances.select_nth_unstable(n); // sort until nth index

    // Collect closest pairs of boxes into circuits. Map box_id -> circuit_id
    let mut circuits: HashMap<u16, u16> = HashMap::new();
    let mut next_id = 0;
    for dist in &distances[..n] {
        connect_pairs(&mut circuits, &mut next_id, dist.i, dist.j);
    }

    // Determine size of each circuit
    let mut circuit_sizes: HashMap<u16, u16> = HashMap::new();
    for id in circuits.values() {
        *circuit_sizes.entry(*id).or_insert(0) += 1;
    }

    // Multiply the three largest circuits
    let mut sizes: Vec<_> = circuit_sizes.values().collect();
    sizes.sort_by(|a, b| b.cmp(&a));
    sizes.into_iter().take(3).map(|i| *i as u64).product()
}

fn parse_input(input: &str) -> Vec<Box3D> {
    input
        .lines()
        .map(|line| {
            let mut nums = line
                .split(",")
                .map(|x| x.parse::<u32>().expect("Could not parse box position"));
            [
                nums.next().unwrap(),
                nums.next().unwrap(),
                nums.next().unwrap(),
            ]
        })
        .collect()
}

// Smaller than (usize, usize, usize) to better fit in cache
#[derive(Eq, Ord, PartialEq, PartialOrd)]
struct Dist {
    d: u64,
    i: u16,
    j: u16,
}

fn calculate_distances(boxes: &[Box3D]) -> Vec<Dist> {
    let n = boxes.len();
    let mut distances = Vec::with_capacity(n * (n - 1) / 2);
    for i in 0..boxes.len() - 1 {
        for j in i + 1..boxes.len() {
            // Dont need to sqrt, order is preserved
            let a = boxes[i];
            let b = boxes[j];
            let d = (a[0].abs_diff(b[0]) as u64).pow(2)
                + (a[1].abs_diff(b[1]) as u64).pow(2)
                + (a[2].abs_diff(b[2]) as u64).pow(2);
            distances.push(Dist {
                d: d,
                i: i as u16,
                j: j as u16,
            });
        }
    }
    distances
}

fn connect_pairs(circuits: &mut HashMap<u16, u16>, next_id: &mut u16, i: u16, j: u16) -> bool {
    match (circuits.get(&i), circuits.get(&j)) {
        (Some(&circuit_i), Some(&circuit_j)) => {
            if circuit_i != circuit_j {
                // Merge circuits
                for v in circuits.values_mut() {
                    if *v == circuit_j {
                        *v = circuit_i;
                    }
                }
                true
            } else {
                false
            }
        }
        (Some(&circuit_i), None) => {
            circuits.insert(j, circuit_i);
            true
        }
        (None, Some(&circuit_j)) => {
            circuits.insert(i, circuit_j);
            true
        }
        _ => {
            circuits.insert(i, *next_id);
            circuits.insert(j, *next_id);
            *next_id += 1;
            true
        }
    }
}

// Keep connecting boxes until they're all in the same circuit.
// What do you get if you multiply together the X coordinates of the last two junction boxes you need to connect?
pub fn part_2(input: &str) -> u64 {
    let boxes = parse_input(input);
    let mut distances = calculate_distances(&boxes);
    // distances.sort_by(|left, right| right.d.cmp(&left.d));
    distances.sort_unstable_by_key(|d| d.d);

    // Disjoint-set/union-find (DSU) approach to connect circuits - https://en.wikipedia.org/wiki/Disjoint-set_data_structure
    // Circuits = vec where each index (junction box) points to the set/circuit it is in
    let mut circuits: Vec<u16> = (0..boxes.len()).map(|i| i as u16).collect();
    let mut sizes: Vec<u16> = vec![1; boxes.len()];
    let mut result = 0;
    for dist in distances {
        let mut circuit_i = find_circuit(dist.i, &mut circuits) as usize;
        let mut circuit_j = find_circuit(dist.j, &mut circuits) as usize;

        if circuit_i != circuit_j {
            if sizes[circuit_i] < sizes[circuit_j] {
                swap(&mut circuit_i, &mut circuit_j);
            }

            circuits[circuit_j] = circuit_i as u16;
            sizes[circuit_i] += sizes[circuit_j];
            result = boxes[dist.i as usize][0] as u64 * boxes[dist.j as usize][0] as u64;
        }
    }

    result
}

// Non-recursive find with path compression
#[inline(always)]
fn find_circuit(i: u16, circuits: &mut [u16]) -> u16 {
    // Find root (= circuit_id)
    let mut root = i;
    while circuits[root as usize] != root {
        root = circuits[root as usize];
    }

    // Set all nodes in the path to the root value
    let mut x = i;
    while circuits[x as usize] != x {
        let next = circuits[x as usize];
        circuits[x as usize] = root;
        x = next;
    }
    root
}

#[cfg(test)]
mod tests {
    use crate::util::util::load_input;

    use super::*;

    const EXAMPLE_INPUT_1: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1_for_n_pairs(EXAMPLE_INPUT_1, 10), 40);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&load_input("inputs/2025/day_8")), 121770);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT_1), 25272);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&load_input("inputs/2025/day_8")), 7893123992);
    }
}
