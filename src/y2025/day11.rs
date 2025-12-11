// How many different paths lead from `you` to `out`?
pub fn part_1(input: &str) -> usize {
    /*
    Input is a directed graph
    Visualized graphs of example and real data shows no cycles
    Real data represents massive directed graph with a few fully connected layers of 3 to 5 nodes
    Between these fully connected layers are about 100 nodes forming many paths
    Paths between nodes can be different lengths
    The `you` node is in the last connected layer.
    Path length from `you` to `out` seems to be 7. Simple DFS should be fine for part 1.
    */
    let (graph, mapping) = parse_graph(input);
    nr_of_paths(&graph, &mapping, "you", "out")
}

fn parse_graph(input: &str) -> (Vec<Vec<usize>>, Vec<&str>) {
    // Map each str to a usize
    let mut mapping: Vec<&str> = Vec::new();
    for line in input.lines() {
        let (source, targets) = line.split_once(": ").unwrap();
        if !mapping.contains(&source) {
            mapping.push(source);
        }

        for target in targets.split_whitespace() {
            if !mapping.contains(&target) {
                mapping.push(target);
            }
        }
    }

    // Each position in graph vec maps the index of the source to the indices of the targets
    let mut graph: Vec<Vec<usize>> = vec![vec![]; mapping.len()];
    for line in input.lines() {
        let (source, targets) = line.split_once(": ").unwrap();
        let source_index = mapping.iter().position(|m| *m == source).unwrap();

        let targets = targets
            .split_whitespace()
            .map(|target| mapping.iter().position(|m| *m == target).unwrap())
            .collect();

        graph[source_index] = targets;
    }
    (graph, mapping)
}

fn nr_of_paths(graph: &Vec<Vec<usize>>, mapping: &Vec<&str>, source: &str, target: &str) -> usize {
    let source = mapping.iter().position(|m| *m == source).unwrap();
    let target = mapping.iter().position(|m| *m == target).unwrap();
    // Since it's directed without cycles, and we want all possible paths, can just keep a list of nodes currently being visited
    let mut paths: Vec<usize> = Vec::from([source]);
    // Visit all states after target. Don't want to explore those
    let mut visited: Vec<u8> = vec![0; mapping.len()];
    let mut after_target = vec![target];
    while let Some(node) = after_target.pop() {
        if visited[node] == 0 {
            visited[node] = 1;
            for next in graph[node].iter() {
                after_target.push(*next);
            }
        }
    }

    let mut result = 0;
    while let Some(node) = paths.pop() {
        for next in graph[node].iter() {
            if *next == target {
                result += 1;
            } else if visited[*next] == 0 {
                paths.push(*next);
            }
        }
    }
    result
}

// Now find the number of paths that lead from `svr` to `out` while passing through both `dac` and `fft`
pub fn part_2(input: &str) -> usize {
    // svr is the first node in the graph
    // fft is between the second and third fully connected layers
    // dac is between layers 5 and 6
    let (graph, mapping) = parse_graph(input);
    let first = nr_of_paths(&graph, &mapping, "svr", "fft");
    // println!(" first: {}", first);
    let second = nr_of_paths(&graph, &mapping, "fft", "dac");
    // println!(" second: {}", second);
    let third = nr_of_paths(&graph, &mapping, "dac", "out");
    // println!(" third: {}", third);
    first * second * third
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

    const EXAMPLE_INPUT_2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT_1), 5);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&load_input("inputs/2025/day_11")), 733);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT_2), 2);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(&load_input("inputs/2025/day_11")),
            290_219_757_077_250
        );
    }
}
