use crate::days::util::load_input;
use crate::{Solution, SolutionPair};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::str::Lines;
use std::usize;

// Modules communicate using pulses. Each pulse is either a high pulse or a low pulse.
// When a module sends a pulse, it sends that type of pulse to each module in its list of destination modules.
#[derive(Debug, PartialEq, Eq, Hash)]
enum Pulse {
    High,
    Low,
}

// The machines are far apart and wired together with long cables.
// The cables don't connect to the machines directly, but rather
// to communication modules attached to the machines that perform
// various initialization tasks and also act as communication relays.
trait CommunicationModule {
    // Modules communicate using pulses. Each pulse is either a high
    // pulse or a low pulse. When a module sends a pulse, it sends
    // that type of pulse to each module in its list of destination modules.
    fn process_pulse(&mut self, name: String, pulse: Pulse) -> Option<Pulse>;

    fn register_input_module(&mut self, name: String);
}

// Flip-flop modules are either on or off; they are initially off.
#[derive(Debug, PartialEq, Eq, Hash)]
struct FlipFlop {
    on: bool,
}

impl FlipFlop {
    fn new() -> Self {
        FlipFlop { on: false }
    }
}

impl CommunicationModule for FlipFlop {
    // If a flip-flop module receives a high pulse, it is ignored and nothing happens.
    // However, if a flip-flop module receives a low pulse, it flips between on and off.
    // If it was off, it turns on and sends a high pulse. If it was on, it turns off and sends a low pulse.
    fn process_pulse(&mut self, _name: String, pulse: Pulse) -> Option<Pulse> {
        match pulse {
            Pulse::High => None,
            Pulse::Low => {
                let was_on = self.on;
                self.on = !self.on;
                if was_on {
                    Some(Pulse::Low)
                } else {
                    Some(Pulse::High)
                }
            }
        }
    }

    fn register_input_module(&mut self, _name: String) {}
}

// Conjunction modules remember the type of the most recent pulse received from each of their
// connected input modules; they initially default to remembering a low pulse for each input.
#[derive(Debug, PartialEq, Eq)]
struct Conjunction {
    memory: HashMap<String, Pulse>,
}

impl Conjunction {
    fn new() -> Self {
        Conjunction {
            memory: HashMap::new(),
        }
    }
}

impl CommunicationModule for Conjunction {
    // When a pulse is received, the conjunction module first updates its memory for that input.
    // Then, if it remembers high pulses for all inputs, it sends a low pulse; otherwise, it sends a high pulse.
    fn process_pulse(&mut self, name: String, pulse: Pulse) -> Option<Pulse> {
        self.memory.insert(name, pulse);

        if self.memory.values().all(|p| *p == Pulse::High) {
            Some(Pulse::Low)
        } else {
            Some(Pulse::High)
        }
    }

    fn register_input_module(&mut self, name: String) {
        self.memory.insert(name, Pulse::Low);
    }
}

impl Hash for Conjunction {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut hasher = DefaultHasher::new();
        for (key, _) in &self.memory {
            key.hash(&mut hasher);
        }
        state.write_u64(hasher.finish());
    }
}

fn part_1(lines: Lines) -> usize {
    let mut modules: HashMap<String, Rc<dyn CommunicationModule>> = HashMap::new();
    // let mut flip_flops: HashMap<String, Rc<FlipFlop>, Vec<String>> = HashMap::new();
    // let mut conjunctions: HashMap<String, Rc<Conjunction>, Vec<String>> = HashMap::new();
    let mut module_destinations: HashMap<String, Vec<String>> = HashMap::new();

    // There is a single broadcast module. When it receives a pulse,
    // it sends the same pulse to all of its destination modules.
    let mut broadcaster: Vec<String> = Vec::new();

    lines.for_each(|line| match line.split(" -> ").collect::<Vec<&str>>()[..] {
        [module, destinations] => {
            let destinations = destinations
                .split(", ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            match module.split_at(1) {
                ("%", name) => {
                    let ff = Rc::new(FlipFlop::new());
                    modules.insert(name.to_string(), ff.clone());
                    module_destinations.insert(name.to_string(), destinations);
                }
                ("&", name) => {
                    let con = Rc::new(Conjunction::new());
                    modules.insert(name.to_string(), con.clone());
                    module_destinations.insert(name.to_string(), destinations);
                }
                ("b", _) => {
                    broadcaster = destinations;
                }
                _ => panic!("Invalid module {:?} in line", line),
            }
        }
        _ => panic!("Invalid line {:?}", line),
    });

    // module_destinations.iter().for_each(|(name, destinations)| {
    //     destinations.iter().for_each(|d| {
    //         modules
    //             .get(d)
    //             .unwrap()
    //             .borrow_mut()
    //             .register_input_module(name.clone())
    //     })
    // });

    // dbg!(modules.keys().collect::<Vec<&String>>());
    modules
        .iter()
        .for_each(|(name, module)| println!("Module {:?}", name));
    dbg!(broadcaster);
    0
}

fn part_2(_lines: Lines) -> usize {
    0
}

pub fn solve() -> SolutionPair {
    let input = load_input("inputs/day_20");
    (
        Solution::from(part_1(input.lines())),
        Solution::from(part_2(input.lines())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_1: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    const EXAMPLE_INPUT_2: &str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

    #[test]
    fn test_part_1_example_1() {
        assert_eq!(part_1(EXAMPLE_INPUT_1.lines()), 0);
    }

    #[test]
    fn test_part_1_example_2() {
        assert_eq!(part_1(EXAMPLE_INPUT_2.lines()), 0);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(load_input("inputs/day_20").lines()), 0);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT_1.lines()), 0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(load_input("inputs/day_20").lines()), 0);
    }
}
