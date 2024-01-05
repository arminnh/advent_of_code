use crate::days::util::load_input;
use crate::{Solution, SolutionPair};
use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::str::Lines;

// Modules communicate using pulses. Each pulse is either a high pulse or a low pulse.
// When a module sends a pulse, it sends that type of pulse to each module in its list of destination modules.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Pulse {
    High,
    Low,
}

// The machines are far apart and wired together with long cables.
// The cables don't connect to the machines directly, but rather
// to communication modules attached to the machines that perform
// various initialization tasks and also act as communication relays.
trait CommunicationModule: fmt::Debug {
    // Modules communicate using pulses. Each pulse is either a high
    // pulse or a low pulse. When a module sends a pulse, it sends
    // that type of pulse to each module in its list of destination modules.
    fn process_pulse(&mut self, source_module_id: usize, pulse: Pulse) -> Option<Pulse>;

    fn register_input_module(&mut self, module_id: usize);
}

// A sink has no state and does nothing with signals
#[derive(Debug, PartialEq, Eq)]
struct Sink {}

impl CommunicationModule for Sink {
    fn process_pulse(&mut self, _module_id: usize, _pulse: Pulse) -> Option<Pulse> {
        None
    }

    fn register_input_module(&mut self, _: usize) {}
}

// When a broadcaster receives a pulse, it sends the same pulse to all of its destination modules.
type Broadcaster = Vec<usize>;

// Flip-flop modules are either on or off; they are initially off.
#[derive(Debug, PartialEq, Eq)]
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
    fn process_pulse(&mut self, _module_id: usize, pulse: Pulse) -> Option<Pulse> {
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

    fn register_input_module(&mut self, _: usize) {}
}

// Conjunction modules remember the type of the most recent pulse received from each of their
// connected input modules; they initially default to remembering a low pulse for each input.
#[derive(Debug, PartialEq, Eq)]
struct Conjunction {
    memory: HashMap<usize, Pulse>,
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
    fn process_pulse(&mut self, source_module_id: usize, pulse: Pulse) -> Option<Pulse> {
        self.memory.insert(source_module_id, pulse);

        if self.memory.values().all(|p| *p == Pulse::High) {
            Some(Pulse::Low)
        } else {
            Some(Pulse::High)
        }
    }

    fn register_input_module(&mut self, module_id: usize) {
        self.memory.insert(module_id, Pulse::Low);
    }
}

fn parse_input(
    lines: Lines,
) -> (
    Vec<Box<dyn CommunicationModule>>,
    Vec<Vec<usize>>,
    Broadcaster,
) {
    // All modules are stored in a vec. Their index in the vec is their ID.
    let mut modules: Vec<Box<dyn CommunicationModule>> = Vec::new();
    // Vec of module names to be able to map module names to module IDs/indexes.
    let mut module_names: Vec<String> = Vec::new();
    // Map of module names to list of names of their destinations. After all
    // input is parsed, this is converted to a nested vec of module IDs for easy indexing.
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
                    modules.push(Box::new(FlipFlop::new()));
                    module_names.push(name.to_string());
                    module_destinations.insert(name.to_string(), destinations);
                }
                ("&", name) => {
                    modules.push(Box::new(Conjunction::new()));
                    module_names.push(name.to_string());
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

    // After parsing, convert broadcaster to vec of module IDs
    let broadcaster: Broadcaster = broadcaster
        .iter()
        .map(|name| module_names.iter().position(|n| n == name).unwrap())
        .collect();

    // After parsing, convert module destinations to a nested vec of module IDs.
    // Each entry contains the destinations for the module at the index of the entry.
    let module_destinations: Vec<Vec<usize>> = module_names
        .iter()
        .map(|name| {
            module_destinations
                .get(name)
                .unwrap()
                .iter()
                .map(|destination| {
                    // If destination does not exist, create a sink module so that pulses can still be sent and counted.
                    if let Some(index) = module_names.iter().position(|n| n == destination) {
                        index
                    } else {
                        modules.push(Box::new(Sink {}));
                        modules.len() - 1
                    }
                })
                .collect()
        })
        .collect();

    // Register input modules to initialize the memory of the Conjunction modules.
    module_destinations
        .iter()
        .enumerate()
        .for_each(|(source_index, destinations)| {
            destinations.iter().for_each(|&destination_index| {
                modules[destination_index].register_input_module(source_index);
            });
        });

    (modules, module_destinations, broadcaster)
}

// At Desert Machine Headquarters, there is a module with a single button on it called, aptly, the button module.
// When you push the button, a single low pulse is sent directly to the broadcaster module.
// After pushing the button, you must wait until all pulses have been delivered and fully handled
// before pushing it again. Never push the button if modules are still processing pulses.
// Returns the amounts of low and high pulses sent in total for the button press.
fn handle_button_press(
    modules: &mut Vec<Box<dyn CommunicationModule>>,
    module_destinations: &Vec<Vec<usize>>,
    broadcaster: &Broadcaster,
) -> (usize, usize) {
    let mut low_pulse_count = 1;
    let mut high_pulse_count = 0;
    // Each signal is pushed to the back of the signals queue to ensure correct order of processing
    let mut signals: VecDeque<(usize, usize, Pulse)> = broadcaster
        .iter()
        .map(|&destination| (0, destination, Pulse::Low))
        .collect();

    while let Some((source, destination, pulse)) = signals.pop_front() {
        // println!("{source} ---{pulse:?}--> {destination}");
        match pulse {
            Pulse::High => high_pulse_count += 1,
            Pulse::Low => low_pulse_count += 1,
        };

        if let Some(new_pulse) = modules[destination].process_pulse(source, pulse) {
            module_destinations[destination]
                .iter()
                .for_each(|&new_destination| {
                    signals.push_back((destination, new_destination, new_pulse))
                });
        }
    }

    (low_pulse_count, high_pulse_count)
}

// Determine the number of low pulses and high pulses that would be sent after pushing the button 1000 times,
// waiting for all pulses to be fully handled after each push of the button. What do you get if you multiply
// the total number of low pulses sent by the total number of high pulses sent?
fn part_1(lines: Lines) -> usize {
    let (mut modules, module_destinations, broadcaster) = parse_input(lines);

    // 1000 is low enough to brute force quickly
    let (low_pulses, high_pulses) = (0..1000).fold((0, 0), |(low_count, high_count), _| {
        let (low, high) = handle_button_press(&mut modules, &module_destinations, &broadcaster);
        // println!("{}, {}", low_count + low, high_count + high);
        (low_count + low, high_count + high)
    });

    low_pulses * high_pulses
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
        assert_eq!(part_1(EXAMPLE_INPUT_1.lines()), 32_000_000);
    }

    #[test]
    fn test_part_1_example_2() {
        assert_eq!(part_1(EXAMPLE_INPUT_2.lines()), 11_687_500);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(load_input("inputs/day_20").lines()), 912_199_500);
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
