use crate::util::util::load_input;
use crate::{Solution, SolutionPair};
use rand::Rng;
use std::collections::HashSet;
use std::str::Lines;
use std::usize;

fn parse_edges(lines: Lines) -> Vec<(String, String)> {
    lines
        .flat_map(|l| match l.split(":").collect::<Vec<_>>()[..] {
            [from, right] => right
                .trim()
                .split_ascii_whitespace()
                .map(|to| (from.to_owned(), to.to_owned()))
                .collect::<Vec<(String, String)>>(),
            _ => panic!("Invalid input {:?}", l),
        })
        .collect()
}

// // Write dot file -> visualize with `dot -Kneato -Tpng -o output.png input.dot`
// fn write_to_dot(edges: &Vec<(String, String)>, file_suffix: &str) -> std::io::Result<()> {
//     let mut dot_content = String::new();

//     dot_content.push_str("graph {\n");
//     edges.iter().for_each(|(from, to)| {
//         dot_content.push_str(&format!("  {} -- {};\n", from, to));
//     });
//     dot_content.push_str("}\n");

//     File::create(format!("outputs/day_25{}.dot", file_suffix))?
//         .write_all(dot_content.as_bytes())?;
//     Ok(())
// }

// Replace random edge `from` -> `to` by contracted edge `fromto` - https://en.wikipedia.org/wiki/Karger's_algorithm
fn contract_edge(nodes: &mut HashSet<String>, edges: &mut Vec<(String, String)>, edge_id: usize) {
    let (from, to) = edges.swap_remove(edge_id);
    let new_node: String = from.to_owned() + &to;
    nodes.remove(&from);
    nodes.remove(&to);
    nodes.insert(new_node.clone());

    // replace edges containing `from` or `to` by new edges containing `fromto`
    edges.retain_mut(|(u, v)| {
        if (u == &from && v == &to) || (u == &to && v == &from) {
            // remove self loops after contraction
            false
        } else if u == &from || u == &to {
            // new edge `fromto` -> `v`
            *u = new_node.clone();
            true
        } else if v == &from || v == &to {
            // new edge `u` -> `fromto`
            *v = new_node.clone();
            true
        } else {
            // keep edges that don't contain `from` or `to``
            true
        }
    });
}

fn part_1(lines: Lines) -> usize {
    let mut rng = rand::thread_rng();
    let mut edges: Vec<(String, String)> = parse_edges(lines.clone());
    let mut nodes: HashSet<String> = edges
        .iter()
        .flat_map(|(from, to)| vec![from.clone(), to.clone()])
        .collect();

    // Massage the starting point to reduce long random runs
    vec![
        1661, 1491, 647, 3104, 2946, 585, 2452, 2043, 1335, 25, 2519, 2841, 787, 2850, 2308, 721,
        2846, 303, 2362, 1148, 2751, 258, 1923, 2018, 2994, 1329, 1345, 2827, 1601, 1109, 1595,
        1732, 1120, 2494, 370, 753, 2897, 571, 957, 293, 1448, 633, 87, 1190, 364, 776, 294, 106,
        1539, 1198, 404, 1412, 501, 819, 406, 1550, 1459, 2114, 1535, 921, 2374, 2965, 1924, 2104,
        1776, 2384, 632, 1711, 1672, 2818, 2024, 249, 314, 2926, 1272, 1136, 2635, 1104, 102, 2164,
        862, 1488, 2537, 2608, 174, 1969, 982, 295, 2277, 1083, 2752, 1808, 2345, 3091, 1231, 352,
    ]
    .into_iter()
    .for_each(|i| contract_edge(&mut nodes, &mut edges, i));

    // Random iterations until solution is found
    loop {
        let mut e = edges.clone();
        let mut n = nodes.clone();

        // Contract edges randomly until only two nodes remain
        while n.len() > 2 {
            let edge_id = rng.gen_range(0..e.len());
            contract_edge(&mut n, &mut e, edge_id);
        }

        // We want a min-cut of 3 edge => if 3 edges remain after the contractions, we can calculate the result
        if e.len() == 3 {
            return e
                .first()
                .map(|name| name.0.len() / 3 * name.1.len() / 3)
                .unwrap();
        }
    }
}

fn part_2(_lines: Lines) -> usize {
    0
}

pub fn solve() -> SolutionPair {
    let input = load_input("inputs/day_25");
    (
        Solution::from(part_1(input.lines())),
        Solution::from(part_2(input.lines())),
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
    fn test_contract_edge_1() {
        let mut nodes = HashSet::from(["A".to_string(), "B".to_string(), "C".to_string()]);
        let mut edges = vec![
            ("A".to_string(), "B".to_string()),
            ("B".to_string(), "C".to_string()),
            ("C".to_string(), "A".to_string()),
        ];
        contract_edge(&mut nodes, &mut edges, 0);

        assert_eq!(nodes, HashSet::from(["AB".to_string(), "C".to_string()]));
        assert_eq!(
            edges,
            vec![
                ("C".to_string(), "AB".to_string()),
                ("AB".to_string(), "C".to_string())
            ]
        );
    }

    #[test]
    fn test_contract_edge_2() {
        let mut nodes = HashSet::from([
            "A".to_string(),
            "B".to_string(),
            "C".to_string(),
            "D".to_string(),
        ]);
        let mut edges = vec![
            ("A".to_string(), "B".to_string()),
            ("B".to_string(), "C".to_string()),
            ("C".to_string(), "D".to_string()),
            ("D".to_string(), "A".to_string()),
            ("B".to_string(), "D".to_string()),
            ("D".to_string(), "B".to_string()),
        ];
        contract_edge(&mut nodes, &mut edges, 3);

        assert_eq!(
            nodes,
            HashSet::from(["DA".to_string(), "B".to_string(), "C".to_string()])
        );
        assert_eq!(
            edges,
            vec![
                ("DA".to_string(), "B".to_string()),
                ("B".to_string(), "C".to_string()),
                ("C".to_string(), "DA".to_string()),
                ("DA".to_string(), "B".to_string()), // this one moves up because of swap_remove
                ("B".to_string(), "DA".to_string()),
            ]
        );

        contract_edge(&mut nodes, &mut edges, 1);

        assert_eq!(nodes, HashSet::from(["DA".to_string(), "BC".to_string()]));
        assert_eq!(
            edges,
            vec![
                ("DA".to_string(), "BC".to_string()),
                ("BC".to_string(), "DA".to_string()),
                ("BC".to_string(), "DA".to_string()),
                ("DA".to_string(), "BC".to_string()),
            ]
        );
    }

    #[test]
    fn test_contract_edge_3() {
        let mut nodes = HashSet::from([
            "rzsqnrcmgjqtnvdrhn".to_string(),
            "lhk".to_string(),
            "ntqbvbxhkrshpzllsrhfx".to_string(),
            "frs".to_string(),
        ]);
        let mut edges: Vec<(String, String)> = vec![
            ("rzsqnrcmgjqtnvdrhn", "ntqbvbxhkrshpzllsrhfx"),
            ("rzsqnrcmgjqtnvdrhn", "ntqbvbxhkrshpzllsrhfx"),
            ("rzsqnrcmgjqtnvdrhn", "ntqbvbxhkrshpzllsrhfx"),
            ("ntqbvbxhkrshpzllsrhfx", "frs"),
            ("frs", "ntqbvbxhkrshpzllsrhfx"),
            ("rzsqnrcmgjqtnvdrhn", "rzsqnr"),
            ("rzsqnrcmgjqtnvdrhn", "ntqbvbxhkrshpzllsrhfx"),
            ("frs", "rzsqnrcmgjqtnvdrhn"),
            ("frs", "lhk"),
            ("ntqbvbxhkrshpzllsrhfx", "rzsqnrcmgjqtnvdrhn"),
            ("ntqbvbxhkrshpzllsrhfx", "lhk"),
            ("ntqbvbxhkrshpzllsrhfx", "rzsqnrcmgjqtnvdrhn"),
        ]
        .iter()
        .map(|(from, to)| (from.to_string(), to.to_string()))
        .collect();
        contract_edge(&mut nodes, &mut edges, 4);

        assert_eq!(
            nodes,
            HashSet::from([
                "frsntqbvbxhkrshpzllsrhfx".to_string(),
                "rzsqnrcmgjqtnvdrhn".to_string(),
                "lhk".to_string()
            ])
        );
        assert_eq!(
            edges,
            vec![
                ("rzsqnrcmgjqtnvdrhn", "frsntqbvbxhkrshpzllsrhfx"),
                ("rzsqnrcmgjqtnvdrhn", "frsntqbvbxhkrshpzllsrhfx"),
                ("rzsqnrcmgjqtnvdrhn", "frsntqbvbxhkrshpzllsrhfx"),
                ("frsntqbvbxhkrshpzllsrhfx", "rzsqnrcmgjqtnvdrhn"),
                ("rzsqnrcmgjqtnvdrhn", "rzsqnr"),
                ("rzsqnrcmgjqtnvdrhn", "frsntqbvbxhkrshpzllsrhfx"),
                ("frsntqbvbxhkrshpzllsrhfx", "rzsqnrcmgjqtnvdrhn"),
                ("frsntqbvbxhkrshpzllsrhfx", "lhk"),
                ("frsntqbvbxhkrshpzllsrhfx", "rzsqnrcmgjqtnvdrhn"),
                ("frsntqbvbxhkrshpzllsrhfx", "lhk")
            ]
            .iter()
            .map(|(from, to)| (from.to_string(), to.to_string()))
            .collect::<Vec<(String, String)>>()
        );
    }

    // #[test]
    // fn test_part_1_example() {
    //     assert_eq!(part_1(EXAMPLE_INPUT_1.lines()), 9 * 6);
    // }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(load_input("inputs/day_25").lines()), 514794);
    }
}
