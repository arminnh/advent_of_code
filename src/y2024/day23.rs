use crate::util::util::load_input;
use crate::{Solution, SolutionPair};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;
use std::str::Lines;
use std::usize;

fn parse_connections(lines: Lines) -> Vec<(&str, &str)> {
    lines
        .map(|line| line.split_once("-").expect("Could not parse connection"))
        .collect()
}

// Connections are bidirectional
fn build_graph(connections: &[(&str, &str)]) -> HashMap<String, HashSet<String>> {
    let mut graph: HashMap<String, HashSet<String>> = HashMap::new();
    for (left, right) in connections {
        let l_str = left.to_string();
        let r_str = right.to_string();
        graph.entry(l_str.clone()).or_default().insert(r_str.clone());
        graph.entry(r_str).or_default().insert(l_str);
    }
    graph
}

// Write dot file -> visualize with `dot -Kneato -Tpng -o output.png input.dot`
fn write_to_dot(edges: &Vec<(String, String)>, file_suffix: &str) -> std::io::Result<()> {
    let mut dot_content = String::new();

    dot_content.push_str("graph {\n");
    edges.iter().for_each(|(from, to)| {
        dot_content.push_str(&format!("  {} -- {};\n", from, to));
    });
    dot_content.push_str("}\n");

    File::create(format!("outputs/2024/day23{}.dot", file_suffix))?
        .write_all(dot_content.as_bytes())?;
    Ok(())
}

// Find all the sets of three inter-connected computers.
// How many contain at least one computer with a name that starts with t?
fn part_1(lines: Lines) -> usize {
    let connections = parse_connections(lines);
    let graph: HashMap<String, HashSet<String>> = build_graph(&connections);
    // write_to_dot(&connections.iter().map(|(l, r)| (l.to_string(), r.to_string())).collect(), "");

    let mut connected_sets: HashSet<Vec<String>> = HashSet::new();
    for (from, targets) in graph.iter().filter(|(k, _)| k.starts_with("t")) {
        let targets: Vec<_> = targets.iter().collect();
        for (i, t) in targets.iter().enumerate() {
            for next_target in &targets[i..] {
                if graph.get(&next_target.to_string()).unwrap().contains(&t.to_string()) {
                    let mut new_set = vec![from.clone(), t.to_string(), next_target.to_string()];
                    new_set.sort();
                    connected_sets.insert(new_set);
                }
            }
        }
    }

    println!("{:#?}", connected_sets);
    connected_sets.len()
}

fn part_2(lines: Lines) -> usize {
    0
}

pub fn solve() -> SolutionPair {
    let input = load_input("inputs/2024/day_23");
    (
        Solution::from(part_1(input.lines())),
        Solution::from(part_2(input.lines())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT.lines()), 7);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(load_input("inputs/2024/day_23").lines()), 1151);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT.lines()), 0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(load_input("inputs/2024/day_23").lines()), 0)
    }
}
