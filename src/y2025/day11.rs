use std::collections::HashMap;

// How many different paths lead from `you` to `out`?
pub fn part_1(input: &str) -> usize {
    /*
    Input is a directed graph
    Visualized graphs of example and real data shows no cycles
    Real data represents massive directed graph with a few fully connected layers of 3 to 5 nodes
    Between these fully connected layers are about 100 nodes forming many paths
    The `you` node is in the last connected layer.
    Path length from `you` to `out` seems to be 7. Simple DFS should be fine for part 1.
    */
    let graph: HashMap<&str, Vec<&str>> = input
        .lines()
        .map(|line| {
            let (source, targets) = line.split_once(": ").unwrap();
            (
                source,
                targets
                    .split_whitespace()
                    .map(|target| target)
                    .collect::<Vec<&str>>(),
            )
        })
        .collect();

    // Since it's directed without cycles, and we want all possible paths, can just keep a list of nodes currently being visited
    let mut paths: Vec<&str> = Vec::from(["you"]);
    let mut result = 0;
    while let Some(node) = paths.pop() {
        if let Some(next_nodes) = graph.get(node) {
            for next in next_nodes {
                if *next == "out" {
                    result += 1;
                } else {
                    paths.push(next);
                }
            }
        }
    }
    result
}

//
pub fn part_2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use crate::util::util::load_input;

    use super::*;

    const EXAMPLE_INPUT_1: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT_1), 5);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&load_input("inputs/2025/day_11")), 733);
    }

    // #[test]
    // fn test_part_2_example() {
    //     assert_eq!(part_2(EXAMPLE_INPUT_1), 0);
    // }

    // #[test]
    // fn test_part_2() {
    //     assert_eq!(part_2(&load_input("inputs/2025/day_11")), 0);
    // }
}
