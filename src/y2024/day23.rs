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
        graph
            .entry(left.to_string())
            .or_default()
            .insert(right.to_string());
        graph
            .entry(right.to_string())
            .or_default()
            .insert(left.to_string());
    }
    graph
}

// Write dot file -> visualize with `dot -Kneato -Tpng -o output.png input.dot`
#[allow(dead_code)]
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

#[allow(dead_code)]
fn write_graph_to_dot(graph: &HashMap<String, HashSet<String>>) {
    let edges: HashSet<(String, String)> = graph
        .iter()
        .flat_map(|(l, r)| {
            r.iter().map(|n| {
                let mut edge = vec![l.to_string(), n.to_string()];
                edge.sort();
                (
                    edge.get(0).unwrap().to_string(),
                    edge.get(1).unwrap().to_string(),
                )
            })
        })
        .collect();
    let _ = write_to_dot(&edges.into_iter().collect(), "");
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
                if graph
                    .get(&next_target.to_string())
                    .unwrap()
                    .contains(&t.to_string())
                {
                    let mut new_set = vec![from.clone(), t.to_string(), next_target.to_string()];
                    new_set.sort();
                    connected_sets.insert(new_set);
                }
            }
        }
    }
    connected_sets.len()
}

// Find largest set of computers that are all connected to each other
// -> Maximal clique problem: https://en.wikipedia.org/wiki/Clique_(graph_theory)
// Could search from the biggest possible group size -> start from nodes with largest nr of neighbors
// Turned out all nodes have the same amount of neighbors, so makes no difference to sort by nr of neighbors
// Use trick of part 1 to filter out edges that prevent creation of a clique of 3 -> you end up with "islands" of nodes
// Can then start searching from the nodes with largest amounts of neighbors
// If all nodes of a connected component contain the same number of neighbors -> found the max clique
fn part_2(lines: Lines) -> String {
    let connections = parse_connections(lines);
    let mut graph: HashMap<String, HashSet<String>> = build_graph(&connections);
    // write_to_dot(&connections.iter().map(|(l, r)| (l.to_string(), r.to_string())).collect(), "");

    // Remove all edges which do not result in a set of three connected computers
    let mut to_remove = Vec::new();
    for (from, neighbors) in &graph {
        for to in neighbors.iter() {
            let neighbors_of_to = graph.get(to).unwrap();
            // If none of the neighbors of 'to' point back to 'from', can remove the connection
            if !neighbors_of_to.iter().any(|n| {
                graph
                    .get(&n.to_string())
                    .unwrap()
                    .contains(&from.to_string())
            }) {
                to_remove.push((from.to_string(), to.to_string()));
                break;
            }
        }
    }
    // println!("Removing {} edges", to_remove.len());
    for (from, to) in to_remove {
        // println!("{} had {} neighbors",from,graph.get(&from.to_string()).unwrap().len());
        // println!("{} had {} neighbors",to,graph.get(&to.to_string()).unwrap().len());
        graph.entry(from.to_string()).and_modify(|n| {
            n.remove(&to.to_string());
        });
        graph.entry(to.to_string()).and_modify(|n| {
            n.remove(&from.to_string());
        });
    }
    // write_graph_to_dot(&graph);

    let mut seen: HashSet<Vec<String>> = HashSet::new();
    let mut items: Vec<(String, HashSet<String>)> = graph.clone().into_iter().collect();
    items.sort_by(|a, b| b.1.len().cmp(&a.1.len()));
    for (from, targets) in &items {
        let mut group: Vec<String> = targets.clone().into_iter().collect();
        group.push(from.clone());
        group.sort();
        // Only check each potential group once
        if !seen.insert(group.clone()) {
            continue;
        }
        // println!("Checking potential group of size {} starting from {}: {:?}", targets.len(), from, &group.join(","));
        // If all nodes of the group have the same amount of neighbors, return the result
        let nrs_of_neighbors = group
            .iter()
            .map(|node| graph.get(node).unwrap().len())
            .collect::<Vec<_>>();
        let nr = nrs_of_neighbors[0];
        if nrs_of_neighbors.iter().all(|x| *x == nr) {
            return group.join(",");
        }
    }

    "No result!".to_string()
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
        assert_eq!(part_2(EXAMPLE_INPUT.lines()), "co,de,ka,ta".to_string());
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(load_input("inputs/2024/day_23").lines()),
            "ar,cd,hl,iw,jm,ku,qo,rz,vo,xe,xm,xv,ys".to_string()
        )
    }
}
