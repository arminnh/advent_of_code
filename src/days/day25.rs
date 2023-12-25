use crate::days::util::load_input;
use crate::{Solution, SolutionPair};
use rand::Rng;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::str::Lines;
use std::usize;

#[derive(Clone)]
struct Graph {
    edges: HashMap<String, Vec<String>>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            edges: HashMap::new(),
        }
    }

    fn add_edge(&mut self, u: String, v: String) {
        self.edges.entry(u.clone()).or_default().push(v.clone());
        self.edges.entry(v).or_default().push(u);
    }

    fn edges_vec(&self) -> Vec<(String, String)> {
        self.edges
            .iter()
            .flat_map(|(from, to)| to.into_iter().map(|to| (from.clone(), to.clone())))
            .collect()
    }

    // Contract edges for nodes u, v into edges for combined node uv
    fn contract_edges(&mut self, u: String, v: String) {
        let new_node: String = u.to_owned() + &v;

        let u_neighbors = self.edges.remove(&u).unwrap();
        let v_neighbors = self.edges.remove(&v).unwrap();
        u_neighbors
            .into_iter()
            .chain(v_neighbors.into_iter())
            .filter(|x| x != &v && x != &u)
            .for_each(|neighbor| {
                self.add_edge(new_node.clone(), neighbor.clone());
                self.edges
                    .get_mut(&neighbor)
                    .unwrap()
                    .retain(|x| x != &u && x != &v);
            });
    }

    #[allow(dead_code)]
    fn print(&self) {
        self.edges
            .iter()
            .for_each(|(from, to)| println!("{} -> {:?}", from, to));
        println!("");
    }

    #[allow(dead_code)]
    // Write dot file -> visualize with `dot -Kneato -Tpng -o output.png input.dot`
    fn write_to_dot(&self, file_suffix: &str) -> std::io::Result<()> {
        let mut dot_content = String::new();

        dot_content.push_str("graph {\n");
        self.edges.iter().for_each(|(from, targets)| {
            targets.iter().for_each(|to| {
                dot_content.push_str(&format!("  {} -- {};\n", from, to));
            });
        });
        dot_content.push_str("}\n");

        File::create(format!("outputs/day_25{}.dot", file_suffix))?
            .write_all(dot_content.as_bytes())?;
        Ok(())
    }
}

fn parse_input(lines: Lines) -> Graph {
    let mut g: Graph = Graph::new();

    lines.for_each(|l| match l.split(":").collect::<Vec<_>>()[..] {
        [from, right] => right
            .trim()
            .split_ascii_whitespace()
            .collect::<Vec<_>>()
            .iter()
            .for_each(|to| g.add_edge(from.to_string(), to.to_string())),
        _ => panic!("Invalid input {:?}", l),
    });

    g
}

fn part_1(lines: Lines) -> usize {
    let graph = parse_input(lines.clone());
    let mut rng = rand::thread_rng();

    loop {
        let mut g = graph.clone();

        while g.edges.len() > 2 {
            let mut edges: Vec<(String, String)> = g.edges_vec();
            let (from, to) = edges.remove(rng.gen_range(0..edges.len()));
            g.contract_edges(from, to);
        }

        let result = g
            .edges
            .keys()
            .map(|name| name.len() / 3)
            .fold(1, |result, x| result * x);

        println!("RESULT {}", result);
    }
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
        assert_eq!(part_1(EXAMPLE_INPUT_1.lines()), 9 * 6);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(load_input("inputs/day_25").lines()), 514794);
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

// fn edges_to_adjacency_matrix(nodes: &Vec<String>, edges: &HashMap<String, Vec<String>>) -> Vec<Vec<bool>> {
//     let mut adj_matrix: Vec<Vec<bool>> = Vec::with_capacity(nodes.len());
//     (0..nodes.len()).for_each(|_| adj_matrix.push(vec![false; nodes.len()]));

//     for (from_id, from_node) in nodes.iter().enumerate() {
//         for to_node in edges[from_node].iter() {
//             let to_id = nodes.iter().position(|n| n == to_node).unwrap();
//             adj_matrix[from_id][to_id] = true;
//         }
//     }
//     // adj_matrix.iter().for_each(|l| {
//     //     l.iter().for_each(|c| print!("{}, ", *c as i32));
//     //     println!("")
//     // });
//     adj_matrix
// }

// #[allow(dead_code)]
// fn write_adjacency_matrix_to_dot(nodes: &Vec<String>, adj_matrix: &[Vec<bool>], file_suffix: &str) -> std::io::Result<()> {
//     let mut dot_content = String::new();

//     dot_content.push_str("graph {\n");
//     for (i, row) in adj_matrix.iter().enumerate() {
//         for (j, &col) in row.iter().enumerate().skip(i + 1) {
//             if col {
//                 dot_content.push_str(&format!("  {} -- {};\n", nodes[i], nodes[j]));
//             }
//         }
//     }
//     dot_content.push_str("}\n");

//     File::create(format!("outputs/day_25{}.dot", file_suffix))?.write_all(dot_content.as_bytes())?;
//     Ok(())
// }
