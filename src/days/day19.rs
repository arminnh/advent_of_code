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
    True,
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
            Condition::True => true,
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
                condition: Condition::True,
                decision: Decision::from_str(dec),
            },
            _ => panic!("Invalid rule {s:?}"),
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

fn part_2(_lines: String) -> usize {
    0
}

pub fn solve() -> SolutionPair {
    let input = load_input("inputs/day_17");
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
        assert_eq!(part_2(EXAMPLE_INPUT.to_string()), 0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(load_input("inputs/day_19")), 0);
    }
}
