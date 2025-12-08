use std::collections::HashMap;

type Box3D = [usize; 3];

// Connect 1000 pairs of nearest junction boxes. What do you get after multiplying together the sizes of the three largest circuits
pub fn part_1(input: &str) -> usize {
    part_1_for_n_pairs(input, 1000)
}

fn part_1_for_n_pairs(input: &str, n: usize) -> usize {
    let boxes = parse_input(input);
    let mut distances = calculate_distances(boxes);
    distances.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

    // Collect closest pairs of boxes into circuits. Map box_id -> circuit_id
    let mut circuits: HashMap<usize, usize> = HashMap::new();
    let mut next_id = 0;
    for _ in 0..n {
        let (_, i, j) = distances.pop().unwrap();
        connect_pairs(&mut circuits, &mut next_id, i, j);
    }

    // Determine size of each circuit
    let mut circuit_sizes: HashMap<usize, usize> = HashMap::new();
    for id in circuits.values() {
        *circuit_sizes.entry(*id).or_insert(0) += 1;
    }

    // Multiply the three largest circuits
    let mut sizes: Vec<_> = circuit_sizes.values().collect();
    sizes.sort_by(|a, b| b.cmp(&a));
    sizes.into_iter().take(3).product()
}

fn parse_input(input: &str) -> Vec<Box3D> {
    input
        .lines()
        .map(|line| {
            let mut nums = line
                .split(",")
                .map(|x| x.parse::<usize>().expect("Could not parse box position"));
            [
                nums.next().unwrap(),
                nums.next().unwrap(),
                nums.next().unwrap(),
            ]
        })
        .collect()
}

fn calculate_distances(boxes: Vec<Box3D>) -> Vec<(usize, usize, usize)> {
    // let mut distances: BinaryHeap<(f64, usize, usize)> = BinaryHeap::new();
    let mut distances = Vec::new();
    for i in 0..boxes.len() - 1 {
        for j in i + 1..boxes.len() {
            // Dont need to sqrt, order is preserved
            let a = boxes[i];
            let b = boxes[j];
            let d = (a[0].abs_diff(b[0])).pow(2)
                + (a[1].abs_diff(b[1])).pow(2)
                + (a[2].abs_diff(b[2])).pow(2);
            distances.push((d, i, j));
        }
    }
    distances
}

fn connect_pairs(
    circuits: &mut HashMap<usize, usize>,
    next_id: &mut usize,
    i: usize,
    j: usize,
) -> bool {
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
pub fn part_2(input: &str) -> usize {
    let boxes = parse_input(input);
    let mut distances = calculate_distances(boxes.clone());
    distances.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

    let mut circuits: HashMap<usize, usize> = HashMap::new();
    let mut next_id = 0;
    let mut last = (0, 0);
    while let Some((_, i, j)) = distances.pop() {
        let just_connected = connect_pairs(&mut circuits, &mut next_id, i, j);
        if just_connected {
            last = (boxes[i][0], boxes[j][0]);
        }
    }

    last.0 * last.1
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
