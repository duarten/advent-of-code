use std::{
    cmp::{max, min},
    collections::HashMap,
    fs,
    ops::RangeInclusive,
};

struct Rule {
    condition: Option<Cond>,
    destination: String,
}

enum CondType {
    Gt,
    Lt,
}

struct Cond(String, usize, CondType);

impl Rule {
    fn matches(&self, p: &HashMap<String, usize>) -> Option<&str> {
        (match self.condition.as_ref() {
            Some(Cond(cat, value, CondType::Gt)) => p.get(cat).unwrap() > value,
            Some(Cond(cat, value, CondType::Lt)) => p.get(cat).unwrap() < value,
            None => true,
        })
        .then_some(self.destination.as_str())
    }
}

fn main() {
    let input = fs::read_to_string("aoc2023/inputs/day19.input").unwrap();
    let mut input = input.split("\n\n");
    let workflows = parse_workflows(input.next().unwrap());
    let mut sum = 0;
    for part in parse_parts(input.next().unwrap()) {
        let mut workflow = &workflows["in"];
        loop {
            let dest = workflow.iter().find_map(|r| r.matches(&part)).unwrap();
            if dest == "A" {
                sum += part.values().sum::<usize>();
                break;
            } else if dest == "R" {
                break;
            } else {
                workflow = &workflows[dest];
            }
        }
    }
    println!("part 1: {}", sum);
    let ranges = ["x", "m", "a", "s"]
        .into_iter()
        .map(|cat| (cat.to_owned(), 1..=4000))
        .collect();
    println!("part 2: {}", check_ranges("in", ranges, &workflows));
}

fn check_ranges(
    workflow: &str,
    mut ranges: HashMap<String, RangeInclusive<usize>>,
    workflows: &HashMap<String, Vec<Rule>>,
) -> usize {
    let mut sum = 0;
    for r in &workflows[workflow] {
        if let Some(Cond(cat, value, cond)) = r.condition.as_ref() {
            let mut on_match = ranges.clone();
            let match_range = on_match.get_mut(cat).unwrap();
            let range = ranges.get_mut(cat).unwrap();
            if matches!(cond, CondType::Gt) {
                *match_range = max(*match_range.start(), *value + 1)..=*match_range.end();
                *range = *range.start()..=min(*range.end(), *value);
            } else {
                *match_range = *match_range.start()..=min(*match_range.end(), *value - 1);
                *range = max(*value, *range.start())..=*range.end();
            }
            sum += step(r, on_match, workflows);
        } else {
            sum += step(r, ranges.clone(), workflows);
        }
    }
    sum
}

fn step(
    r: &Rule,
    ranges: HashMap<String, RangeInclusive<usize>>,
    workflows: &HashMap<String, Vec<Rule>>,
) -> usize {
    if r.destination == "A" {
        ranges.values().map(|r| r.end() - r.start() + 1).product()
    } else if r.destination != "R" {
        check_ranges(&r.destination, ranges, workflows)
    } else {
        0
    }
}

fn parse_workflows(input: &str) -> HashMap<String, Vec<Rule>> {
    let mut workflows = HashMap::new();
    for workflow in input.split('\n') {
        let rules_start = workflow.find('{').unwrap();
        let name = workflow[0..rules_start].to_owned();
        let mut rules = Vec::new();
        for rule in workflow[rules_start + 1..workflow.len() - 1].split(',') {
            if rule.contains(':') {
                let (rule, dest) = rule.split_once(':').unwrap();
                if let Some((cat, value)) = rule.split_once('<') {
                    rules.push(Rule {
                        condition: Some(Cond(cat.to_owned(), value.parse().unwrap(), CondType::Lt)),
                        destination: dest.to_owned(),
                    });
                } else {
                    let (cat, value) = rule.split_once('>').unwrap();
                    rules.push(Rule {
                        condition: Some(Cond(cat.to_owned(), value.parse().unwrap(), CondType::Gt)),
                        destination: dest.to_owned(),
                    });
                }
            } else {
                rules.push(Rule {
                    condition: None,
                    destination: rule.to_owned(),
                });
            }
        }
        workflows.insert(name, rules);
    }
    workflows
}

fn parse_parts(input: &str) -> Vec<HashMap<String, usize>> {
    let mut parts = Vec::new();
    for part in input.split('\n') {
        let mut cats = HashMap::new();
        for cat in part[1..part.len() - 1].split(',') {
            let (cat, value) = cat.split_once('=').unwrap();
            cats.insert(cat.to_owned(), value.parse().unwrap());
        }
        parts.push(cats);
    }
    parts
}
