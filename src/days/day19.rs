use crate::days::util::load_input;
use crate::{Solution, SolutionPair};
use std::collections::HashMap;
use std::usize;

#[derive(Debug, Clone, Copy)]
enum Category {
    X,
    M,
    A,
    S,
}

impl Category {
    fn from_char(c: char) -> Self {
        match c {
            'x' => Category::X,
            'm' => Category::M,
            'a' => Category::A,
            's' => Category::S,
            _ => panic!("Invalid category {c:?}"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Condition {
    LT(Category, usize),
    GT(Category, usize),
    NOOP,
}

impl Condition {
    fn from_str(s: &str) -> Condition {
        let mut chars = s.chars();
        let category = Category::from_char(chars.next().unwrap());
        let operator = chars.next().unwrap();
        let operand = chars
            .as_str()
            .parse::<usize>()
            .expect("Could not parse condition operand");

        match operator {
            '<' => Condition::LT(category, operand),
            '>' => Condition::GT(category, operand),
            _ => panic!("Invalid condition {s:?}"),
        }
    }
}

#[derive(Debug, Clone)]
enum Decision {
    Accept,
    Reject,
    Transfer(String),
}

impl Decision {
    fn from_str(s: &str) -> Decision {
        match s {
            "A" => Decision::Accept,
            "R" => Decision::Reject,
            _ => Decision::Transfer(s.to_string()),
        }
    }
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn calculate_result(&self) -> usize {
        self.x + self.m + self.a + self.s
    }

    fn get(&self, category: Category) -> usize {
        match category {
            Category::X => self.x,
            Category::M => self.m,
            Category::A => self.a,
            Category::S => self.s,
        }
    }

    fn from_str(line: &str) -> Part {
        match line
            .split_terminator(&['{', ',', '}'][..])
            .skip(1)
            .collect::<Vec<&str>>()
            .as_slice()
        {
            [x, m, a, s] => Part {
                x: x[2..].parse::<usize>().unwrap(),
                m: m[2..].parse::<usize>().unwrap(),
                a: a[2..].parse::<usize>().unwrap(),
                s: s[2..].parse::<usize>().unwrap(),
            },
            _ => panic!("Invalid part {line:?}"),
        }
    }
}

fn parse_parts(parts: &str) -> Vec<Part> {
    parts.lines().map(|line| Part::from_str(line)).collect()
}

#[derive(Debug)]
struct Rule {
    condition: Condition,
    decision: Decision,
}

impl Rule {
    fn process(&self, part: &Part) -> Option<Decision> {
        let condition = match self.condition {
            Condition::LT(category, target) => part.get(category) < target,
            Condition::GT(category, target) => part.get(category) > target,
            Condition::NOOP => true,
        };

        if condition {
            Some(self.decision.clone())
        } else {
            None
        }
    }

    fn from_str(s: &str) -> Rule {
        match s.split(':').collect::<Vec<&str>>()[..] {
            [cond, dec] => Rule {
                condition: Condition::from_str(cond),
                decision: Decision::from_str(dec),
            },
            [dec] => Rule {
                condition: Condition::NOOP,
                decision: Decision::from_str(dec),
            },
            _ => panic!("Invalid rule {s:?}"),
        }
    }

    // For the given ranges (tuples (from, to)) of variables [X, M, A, S], adapt the range
    // this rule operates on into a variant that passes this rule and one that fails this rule.
    fn generate_pass_and_fail_ranges(
        &self,
        part_ranges: [(usize, usize); 4],
    ) -> (Option<[(usize, usize); 4]>, Option<[(usize, usize); 4]>) {
        match self.condition {
            Condition::LT(category, target) => {
                let index = category as usize;
                let (from, to) = part_ranges[index];
                if target <= from {
                    // Range starts higher than target -> can only fail
                    (None, Some(part_ranges))
                } else if target <= to {
                    // Target fits in range -> split the range into pass and fail variants
                    let mut pass = part_ranges.clone();
                    pass[index] = (from, target - 1);
                    let mut fail = part_ranges;
                    fail[index] = (target, to);

                    (Some(pass), Some(fail))
                } else {
                    // Whole range is less than target -> can only pass
                    (Some(part_ranges), None)
                }
            }
            Condition::GT(category, target) => {
                let index = category as usize;
                let (from, to) = part_ranges[index];
                if target < from {
                    // Range starts higher than target -> can only pass
                    (Some(part_ranges), None)
                } else if target < to {
                    // Target fits in range -> split the range into pass and fail variants
                    let mut pass = part_ranges.clone();
                    pass[index] = (target + 1, to);
                    let mut fail = part_ranges;
                    fail[index] = (from, target);

                    (Some(pass), Some(fail))
                } else {
                    // Whole range is less than target -> can only fail
                    (None, Some(part_ranges))
                }
            }
            Condition::NOOP => (Some(part_ranges), None),
        }
    }
}

#[derive(Debug)]
struct Workflow {
    rules: Vec<Rule>,
}

impl Workflow {
    fn process(&self, part: &Part) -> Decision {
        // In a workflow, the last rule is always a no-op that simply results in a decision
        self.rules
            .iter()
            .filter_map(|rule| rule.process(part))
            .next()
            .expect("No result for workflow!")
    }

    fn from_str(s: &str) -> Self {
        Workflow {
            rules: s.split(',').map(|s| Rule::from_str(s)).collect(),
        }
    }
}

type Workflows<'a> = HashMap<&'a str, Workflow>;

fn parse_workflows(workflows: &str) -> Workflows {
    workflows
        .lines()
        .map(|line| {
            match line
                .split_terminator(&['{', '}'][..])
                .collect::<Vec<&str>>()[..]
            {
                [name, rules] => (name, Workflow::from_str(rules)),
                _ => panic!("Invalid workflow line {line:?}"),
            }
        })
        .collect()
}

fn is_part_accepted(workflows: &Workflows, part: &Part) -> bool {
    let mut current_workflow = workflows.get("in").unwrap();
    loop {
        match current_workflow.process(part) {
            Decision::Accept => return true,
            Decision::Reject => return false,
            Decision::Transfer(next) => current_workflow = workflows.get(next.as_str()).unwrap(),
        }
    }
}

// Sort through all of the parts you've been given; what do you get if you add together
// all of the rating numbers for all of the parts that ultimately get accepted?
fn part_1(input: String) -> usize {
    let (workflows, parts) = match input.split("\n\n").collect::<Vec<&str>>()[..] {
        [workflows, parts] => (parse_workflows(workflows), parse_parts(parts)),
        _ => panic!("Invalid input. Could not split workflows and parts."),
    };

    parts
        .into_iter()
        .filter(|part| is_part_accepted(&workflows, part))
        .map(|part| part.calculate_result())
        .sum()
}

// Take all possible flows through the workflows, starting from "in", and generate the ranges of
// parts that eventually get accepted.
fn generate_possible_part_ranges(workflows: &Workflows) -> Vec<[(usize, usize); 4]> {
    let mut result: Vec<[(usize, usize); 4]> = Vec::new();
    let mut candidates: Vec<(String, [(usize, usize); 4])> = Vec::from([(
        "in".to_string(),
        [(1, 4000), (1, 4000), (1, 4000), (1, 4000)],
    )]);

    while let Some((workflow_name, mut candidate)) = candidates.pop() {
        let rules = &workflows.get(workflow_name.as_str()).unwrap().rules;
        for rule in rules {
            // For each rule, generate the two branches that would pass and fail the current rule
            let (pass, fail) = rule.generate_pass_and_fail_ranges(candidate);

            // Handle the branch that passes the current rule
            if let Some(pass) = pass {
                match &rule.decision {
                    // Found an accepted set of ranges!
                    Decision::Accept => result.push(pass),
                    // Do nothing for rejected candidate. Is now popped from the stack.
                    Decision::Reject => (),
                    // Rule transfers to another workflow -> push new candidate to be handled by later rules
                    Decision::Transfer(next) => candidates.push((next.clone(), pass)),
                }
            }

            if let Some(fail) = fail {
                // If there is a failing branch, it is the starting point for the next rule
                candidate = fail
            } else {
                break;
            }
        }
    }

    result
}

// Consider only your list of workflows; the list of part ratings
// that the Elves wanted you to sort is no longer relevant.
// How many distinct combinations of ratings will be accepted by the Elves' workflows?
fn part_2(input: String) -> usize {
    let workflows = match input.split("\n\n").collect::<Vec<&str>>()[..] {
        [workflows, _] => parse_workflows(workflows),
        _ => panic!("Invalid input. Could not split workflows and parts."),
    };

    generate_possible_part_ranges(&workflows)
        .iter()
        .map(|r| r.iter().map(|(from, to)| to - from + 1).product::<usize>())
        .sum()
}

pub fn solve() -> SolutionPair {
    let input = load_input("inputs/day_19");
    (
        Solution::from(part_1(input.clone())),
        Solution::from(part_2(input)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT.to_string()), 19114);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(load_input("inputs/day_19")), 401674);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT.to_string()), 167409079868000);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(load_input("inputs/day_19")), 134906204068564);
    }
}
