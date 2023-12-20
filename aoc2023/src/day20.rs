use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{self, BufRead},
};

use utils::lcm;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Pulse {
    High,
    Low,
}

#[derive(Clone)]
struct Module {
    outputs: Vec<String>,
    kind: ModuleKind,
}

#[derive(Clone)]
enum ModuleKind {
    Broadcast,
    FlipFlop { on: bool },
    Conjunction { remembered: HashMap<String, Pulse> },
}

impl Module {
    fn apply(&mut self, input: &str, pulse: Pulse) -> Option<Pulse> {
        match &mut self.kind {
            ModuleKind::FlipFlop { on } => {
                if pulse == Pulse::High {
                    None
                } else {
                    *on = !*on;
                    Some(if *on { Pulse::High } else { Pulse::Low })
                }
            }
            ModuleKind::Conjunction { remembered } => {
                remembered.insert(input.to_string(), pulse);
                Some(if remembered.values().all(|p| *p == Pulse::High) {
                    Pulse::Low
                } else {
                    Pulse::High
                })
            }
            ModuleKind::Broadcast => Some(pulse),
        }
    }
}

fn main() {
    let file = File::open("aoc2023/inputs/day20.input").unwrap();
    let mut modules = HashMap::new();
    for l in io::BufReader::new(file).lines().map(|l| l.unwrap()) {
        let (input, outputs) = l.split_once(" -> ").unwrap();
        let outputs = outputs
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        if let Some(name) = input.strip_prefix('&') {
            modules.insert(
                name.to_string(),
                Module {
                    outputs,
                    kind: ModuleKind::Conjunction {
                        remembered: HashMap::new(),
                    },
                },
            );
        } else if let Some(name) = input.strip_prefix('%') {
            modules.insert(
                name.to_string(),
                Module {
                    outputs,
                    kind: ModuleKind::FlipFlop { on: false },
                },
            );
        } else {
            modules.insert(
                input.to_string(),
                Module {
                    outputs,
                    kind: ModuleKind::Broadcast,
                },
            );
        }
    }
    let paired_modules = modules
        .iter()
        .flat_map(|(k, v)| v.outputs.iter().map(move |o| (k.to_owned(), o.to_owned())))
        .collect::<Vec<_>>();
    for (input, output) in paired_modules {
        if let Some(module) = modules.get_mut(&output) {
            if let ModuleKind::Conjunction { remembered } = &mut module.kind {
                remembered.insert(input.to_string(), Pulse::Low);
            }
        }
    }

    println!("part 1: {}", cycle(&modules, 1000, None).0);

    let inputs_to_rx = modules
        .iter()
        .find_map(|(name, m)| m.outputs.contains(&"rx".to_owned()).then_some(name))
        .unwrap();
    let cycles = modules
        .iter()
        .filter(|(_, m)| m.outputs.contains(inputs_to_rx))
        .map(|(name, _)| cycle(&modules, usize::MAX, Some(name)))
        .fold(1, |a, (_, b)| lcm(a, b));
    println!("part 2: {}", cycles);
}

fn cycle(
    modules: &HashMap<String, Module>,
    max_presses: usize,
    target: Option<&String>,
) -> (usize, usize) {
    let mut low_pulses = 0;
    let mut high_pulses = 0;
    let mut modules = modules.clone();
    for cycle in 1..=max_presses {
        let mut pulses =
            VecDeque::from([("button".to_string(), Pulse::Low, "broadcaster".to_string())]);
        while let Some((input, pulse, output)) = pulses.pop_front() {
            if pulse == Pulse::High {
                high_pulses += 1;
            } else {
                low_pulses += 1;
            }
            if pulse == Pulse::High && Some(&input) == target {
                return (low_pulses * high_pulses, cycle);
            }
            if let Some(m) = modules.get_mut(&output) {
                if let Some(pulse) = m.apply(&input, pulse) {
                    for o in m.outputs.iter() {
                        pulses.push_back((output.clone(), pulse, o.to_owned()));
                    }
                }
            }
        }
    }
    (low_pulses * high_pulses, max_presses)
}
