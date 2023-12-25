use crate::days::util::load_input;
use crate::{Solution, SolutionPair};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::str::Lines;
use std::usize;

fn parse_edges(lines: Lines) -> HashMap<&str, Vec<&str>> {
    let mut edges: HashMap<&str, Vec<&str>> = HashMap::new();
    lines.for_each(|l| match l.split(":").collect::<Vec<&str>>()[..] {
        [from, right] => right
            .trim()
            .split_ascii_whitespace()
            .collect::<Vec<&str>>()
            .iter()
            .for_each(|to| {
                edges.entry(from).or_default().push(to);
                edges.entry(to).or_default().push(from);
            }),
        _ => panic!("Invalid input {:?}", l),
    });
    edges.iter().for_each(|l| println!("{:?}", l));
    edges
}

fn edges_to_adjacency_matrix(
    nodes: &Vec<&str>,
    edges: &HashMap<&str, Vec<&str>>,
) -> Vec<Vec<bool>> {
    let mut adj_matrix: Vec<Vec<bool>> = Vec::with_capacity(nodes.len());
    (0..nodes.len()).for_each(|_| adj_matrix.push(vec![false; nodes.len()]));

    for (from_id, from_node) in nodes.iter().enumerate() {
        for to_node in edges[from_node].iter() {
            let to_id = nodes.iter().position(|n| n == to_node).unwrap();
            adj_matrix[from_id][to_id] = true;
        }
    }
    // adj_matrix.iter().for_each(|l| {
    //     l.iter().for_each(|c| print!("{}, ", *c as i32));
    //     println!("")
    // });
    adj_matrix
}

#[allow(dead_code)]
// Write dot file -> visualize with `dot -Kneato -Tpng -o output.png input.dot`
fn write_to_dot(nodes: &Vec<&str>, adj_matrix: &[Vec<bool>]) -> std::io::Result<()> {
    let mut dot_content = String::new();

    dot_content.push_str("graph {\n");
    for (i, row) in adj_matrix.iter().enumerate() {
        for (j, &col) in row.iter().enumerate().skip(i + 1){
            if col {
                dot_content.push_str(&format!("  {} -- {};\n", nodes[i], nodes[j]));
            }
        }
    }
    dot_content.push_str("}\n");

    File::create("outputs/day25_.dot")?.write_all(dot_content.as_bytes())?;
    Ok(())
}

fn part_1(lines: Lines) -> usize {
    let edges = parse_edges(lines);
    let nodes: Vec<&str> = edges.keys().copied().collect();
    println!("{:?}", nodes);
    let adj_matrix = edges_to_adjacency_matrix(&nodes, &edges);
    // let _ = write_to_dot(&nodes, &adj_matrix);

    edges.len()
}

fn part_2(_lines: Lines) -> usize {
    0
}

pub fn solve() -> SolutionPair {
    (
        Solution::from(part_1(load_input("inputs/day_25").lines())),
        Solution::from(part_2(load_input("inputs/day_25").lines())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_1: &str = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT_1.lines()), 0);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(load_input("inputs/day_25").lines()), 0);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT_1.lines()), 0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(load_input("inputs/day_25").lines()), 0);
    }
}
