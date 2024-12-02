use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

type Item = usize;

fn parse_items(s: String) -> Result<VecDeque<Item>, Box<dyn Error>> {
    Ok(s.split_once(":")
        .ok_or("Could not split on ':'")?
        .1
        .split(",")
        .map(|i| i.trim().parse().expect("Item is not a valid number."))
        .collect())
}

#[derive(Debug)]
enum Operand {
    Old,
    Num(usize),
}

impl Operand {
    fn from_str(s: &str) -> Result<Self, Box<dyn Error>> {
        if s == "old" {
            Ok(Operand::Old)
        } else {
            Ok(Operand::Num(s.parse()?))
        }
    }
}

#[derive(Debug)]
enum Operator {
    Add,
    Mult,
}

impl Operator {
    fn from_str(s: &str) -> Self {
        if s == "*" {
            Operator::Mult
        } else {
            Operator::Add
        }
    }

    fn execute(&self, x: usize, y: usize) -> usize {
        match self {
            Operator::Add => x + y,
            Operator::Mult => x * y,
        }
    }
}

#[derive(Debug)]
struct Operation {
    operator: Operator,
    left: Operand,
    right: Operand,
}

impl Operation {
    fn from_str(s: String) -> Result<Self, Box<dyn Error>> {
        if let [left, op, right] = s
            .split_once("=")
            .ok_or("Could not split on '='")?
            .1
            .split_ascii_whitespace()
            .collect::<Vec<&str>>()[..]
        {
            Ok(Operation {
                operator: Operator::from_str(op),
                left: Operand::from_str(left)?,
                right: Operand::from_str(right)?,
            })
        } else {
            Err("Invalid operation.".into())
        }
    }

    pub fn execute(&self, old: usize) -> usize {
        match (&self.left, &self.right) {
            (Operand::Old, Operand::Old) => self.operator.execute(old, old),
            (Operand::Old, Operand::Num(num)) => self.operator.execute(old, *num),
            (Operand::Num(num), Operand::Old) => self.operator.execute(*num, old),
            (Operand::Num(left), Operand::Num(right)) => self.operator.execute(*left, *right),
        }
    }
}

#[derive(Debug)]
struct Action {
    denominator: usize,
    target_if_true: i32,
    target_if_false: i32,
}

impl Action {
    fn from_str(s_test: String, s_true: String, s_false: String) -> Result<Self, Box<dyn Error>> {
        Ok(Action {
            denominator: s_test
                .split_once("divisible by")
                .ok_or("Could not split on 'divisible by'")?
                .1
                .trim()
                .parse()?,
            target_if_true: s_true
                .split_once("monkey")
                .ok_or("Could not split on 'monkey'")?
                .1
                .trim()
                .parse()?,
            target_if_false: s_false
                .split_once("monkey")
                .ok_or("Could not split on 'monkey'")?
                .1
                .trim()
                .parse()?,
        })
    }

    pub fn get_target(&self, worry_level: usize) -> &i32 {
        if worry_level % self.denominator == 0 {
            // println!(
            //     "    Current worry level is divisible by {}.",
            //     self.denominator
            // );
            &self.target_if_true
        } else {
            // println!(
            //     "    Current worry level is not divisible by {}.",
            //     self.denominator
            // );
            &self.target_if_false
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<Item>,
    operation: Operation,
    action: Action,
    inspections: usize,
}

struct Monkeys(HashMap<i32, RefCell<Monkey>>);

impl Monkey {
    fn from_str(lines: &mut Lines<BufReader<File>>) -> Result<Self, Box<dyn Error>> {
        Ok(Monkey {
            items: parse_items(lines.next().ok_or("EOL while reading items")??)?,
            operation: Operation::from_str(lines.next().ok_or("EOL while reading operation")??)?,
            action: Action::from_str(
                lines.next().ok_or("EOL while reading test")??,
                lines.next().ok_or("EOL while reading test true case")??,
                lines.next().ok_or("EOL while reading test false case")??,
            )?,
            inspections: 0,
        })
    }

    fn pop_item(&mut self) -> Option<Item> {
        let mut i = self.items.pop_front()?;
        self.inspections += 1;
        // println!("  Monkey inspects an item with a worry level of {}.", i);
        // print!("    Worry level increases from {}", i);
        i = self.operation.execute(i);
        // println!(" to {}.", i);
        Some(i)
    }

    fn push_item(&mut self, item: Item) {
        self.items.push_back(item);
    }

    fn throw_items(&mut self, monkeys: &Monkeys, mod_n: Option<usize>) {
        while let Some(item) = self.pop_item() {
            let item = match mod_n {
                Some(mod_n) => item % mod_n,
                None => item / 3,
            };
            // println!(
            //     "    Monkey gets bored with item. Worry level reduced to {}.",
            //     item
            // );

            let target: &i32 = self.action.get_target(item);
            // println!(
            //     "    Item with worry level {} is thrown to monkey {}.",
            //     item, target
            // );
            monkeys.0.get(target).unwrap().borrow_mut().push_item(item);
        }
    }
}

impl Monkeys {
    fn from_str(mut lines: Lines<BufReader<File>>) -> Monkeys {
        let mut out = HashMap::new();

        while let Some(Ok(monkey_line)) = lines.next() {
            let i: u32 = monkey_line.chars().nth(7).unwrap().to_digit(10).unwrap();
            if let Ok(monkey) = Monkey::from_str(&mut lines) {
                out.insert(i.try_into().unwrap(), RefCell::new(monkey));
            } else {
                print!("COULD NOT LOAD MONKEY {}.", i);
            }
            lines.next();
        }

        Monkeys(out)
    }

    fn print(&self, round: i32) {
        println!(
            "\nAfter round {}, the monkeys are holding items with these worry levels:",
            round
        );
        (0..self.0.len()).for_each(|i| {
            if let Some(monkey) = self.0.get(&i32::try_from(i).unwrap()) {
                println!(
                    "Monkey {i} inspected {} times. Items: {:?}",
                    monkey.borrow().inspections,
                    monkey.borrow().items,
                );
            }
        });
    }

    fn do_rounds(&self, rounds: i32, lowest_common_multiple: Option<usize>) {
        (1..=rounds).for_each(|round| {
            (0..self.0.len()).for_each(|i| {
                if let Some(monkey) = self.0.get(&i32::try_from(i).unwrap()) {
                    monkey
                        .borrow_mut()
                        .throw_items(&self, lowest_common_multiple);
                }
            });
            // self.print(round)
        });
    }

    fn calc_monkey_business(&self) -> usize {
        let mut inspections: Vec<usize> = self
            .0
            .values()
            .into_iter()
            .map(|v| v.borrow().inspections)
            .collect();
        inspections.sort();
        println!("\n\n{inspections:?}");
        let out = inspections[inspections.len() - 2] * inspections[inspections.len() - 1];

        println!("Result: {out}");
        out
    }
}

fn part_1(lines: Lines<BufReader<File>>) -> usize {
    let monkeys: Monkeys = Monkeys::from_str(lines);
    monkeys.do_rounds(20, None);
    monkeys.calc_monkey_business()
}

fn part_2(lines: Lines<BufReader<File>>) -> usize {
    let monkeys: Monkeys = Monkeys::from_str(lines);
    let lowest_common_multiple = Some(
        monkeys
            .0
            .values()
            .into_iter()
            .map(|m| m.borrow().action.denominator)
            .collect::<HashSet<_>>()
            .iter()
            .fold(1, |acc, i| acc * i),
    );
    println!("lowest_common_multiple: {lowest_common_multiple:?}");
    monkeys.do_rounds(10_000, lowest_common_multiple);
    monkeys.calc_monkey_business()
}

fn get_lines(path: &str) -> Lines<BufReader<File>> {
    BufReader::new(File::open(path).expect("Could not open file.")).lines()
}

fn main() {
    // part_1(get_lines("inputs/day_11"));
    part_2(get_lines("inputs/day_11"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(get_lines("inputs/day_11_example")), 10_605)
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(get_lines("inputs/day_11")), 100_345)
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(get_lines("inputs/day_11_example")), 2_713_310_158)
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(get_lines("inputs/day_11")), 28_537_348_205)
    }
}
