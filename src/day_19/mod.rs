use std::{collections::HashMap, fs, ops::Range};

#[derive(Debug)]
struct Part(Vec<(Category, usize)>);
impl Part {
    fn score(&self) -> u32 {
        return self.0.iter().fold(0, |acc, x| acc + x.1 as u32);
    }
}
impl From<&str> for Part {
    fn from(value: &str) -> Self {
        let rest = value.trim_end_matches("}");
        let rest = rest.trim_start_matches("{");
        let part = rest
            .split(",")
            .map(|category| {
                let (category_str, value) = category.split_once("=").unwrap();
                (category_str.into(), value.parse::<usize>().unwrap())
            })
            .collect::<Vec<_>>();
        Part(part)
    }
}
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Category {
    X,
    M,
    A,
    S,
}
impl From<&str> for Category {
    fn from(value: &str) -> Self {
        match value {
            "x" => Self::X,
            "a" => Self::A,
            "m" => Self::M,
            "s" => Self::S,
            _ => {
                panic!("invalid part category")
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Operation {
    Greater,
    Lesser,
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        match value {
            ">" => Operation::Greater,
            "<" => Operation::Lesser,
            _ => {
                panic!("Invalid operation")
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Condition {
    category: Category,
    operation: Operation,
    value: usize,
}
#[derive(Debug, PartialEq, Eq)]
struct Rule {
    dest: String,
    condition: Option<Condition>,
}

impl Rule {
    fn evaluate(&self, part: &Part) -> Option<&str> {
        if self.condition == None {
            return Some(&self.dest);
        }
        let condition = self.condition.unwrap();
        let cat_index = part
            .0
            .iter()
            .position(|(category, _)| *category == condition.category)
            .unwrap();
        match condition.operation {
            Operation::Greater => {
                if part.0[cat_index].1 > condition.value {
                    Some(&self.dest)
                } else {
                    None
                }
            }
            Operation::Lesser => {
                if part.0[cat_index].1 < condition.value {
                    Some(&self.dest)
                } else {
                    None
                }
            }
        }
    }
}

impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        let (condition, dest) = match value.split_once(":") {
            Some((rest, dest)) => {
                let (category, rest) = rest.split_at(1);
                let (operation, value) = rest.split_at(1);
                let value = value.parse::<usize>().unwrap();
                (
                    Some(Condition {
                        operation: operation.into(),
                        value,
                        category: category.into(),
                    }),
                    dest,
                )
            }
            None => (None, value),
        };

        Self {
            dest: String::from(dest),
            condition,
        }
    }
}

struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl From<&str> for Workflow {
    fn from(value: &str) -> Self {
        let (name, rest) = value.split_once("{").unwrap();
        let rest = rest.trim_end_matches("}");
        let rules = rest
            .split(",")
            .map(|rule_str| rule_str.into())
            .collect::<Vec<Rule>>();
        Self {
            name: String::from(name),
            rules,
        }
    }
}

#[allow(dead_code)]
pub fn solve_1() {
    let input = fs::read_to_string("inputs/day_19.txt").unwrap();
    let lines = input.lines().collect::<Vec<_>>();
    let mut iter = lines.into_iter();
    let mut workflows = HashMap::new();
    loop {
        let line = match iter.next() {
            Some(line) => line,
            None => break,
        };

        if line.is_empty() {
            break;
        }
        let workflow: Workflow = (*line).into();
        workflows.insert(workflow.name, workflow.rules);
    }
    let mut parts = vec![];
    loop {
        let line = match iter.next() {
            Some(line) => line,
            None => break,
        };

        let part: Part = (*line).into();
        parts.push(part);
    }

    let start = "in";
    let accepted = "A";
    let rejected = "R";

    let mut res = 0;
    for part in parts {
        let mut workflow_name = start;
        loop {
            let workflow = workflows.get(workflow_name).unwrap();

            for rule in workflow {
                if let Some(dest) = rule.evaluate(&part) {
                    workflow_name = dest;
                    break;
                }
            }
            if workflow_name == accepted {
                let score = part.score();
                res += score;
                break;
            }
            if workflow_name == rejected {
                break;
            }
        }
    }

    println!("res = {res}");
}

#[allow(dead_code)]
pub fn solve_2() {
    let input = fs::read_to_string("inputs/day_19.txt").unwrap();
    let lines = input.lines().collect::<Vec<_>>();
    let mut iter = lines.into_iter();
    let mut workflows = HashMap::new();
    loop {
        let line = match iter.next() {
            Some(line) => line,
            None => break,
        };

        if line.is_empty() {
            break;
        }
        let workflow: Workflow = (*line).into();
        workflows.insert(workflow.name, workflow.rules);
    }
    let start = "in";

    let mut combination = HashMap::from([
        (Category::X, 1..4000),
        (Category::M, 1..4000),
        (Category::A, 1..4000),
        (Category::S, 1..4000),
    ]);

    let res = count_combinations(&mut workflows, &mut combination, &start);

    println!("res = {res}");
}

fn count_combinations(
    workflows: &HashMap<String, Vec<Rule>>,
    combination: &mut HashMap<Category, Range<i32>>,
    current: &str,
) -> u64 {
    let accepted = "A";
    let rejected = "R";

    if current == accepted {
        let score = cal_score(combination);
        return score;
    } else if current == rejected {
        return 0;
    }
    let mut score = 0;
    let rules = workflows.get(current).unwrap();

    let mut nc = combination.clone();

    for rule in rules {
        if let Some(condition) = rule.condition {
            let r = nc.get(&condition.category).unwrap().clone();
            let mut new_range = r.clone();
            match condition.operation {
                Operation::Greater => {
                    new_range.start = condition.value as i32 + 1;
                }
                Operation::Lesser => {
                    new_range.end = condition.value as i32 - 1;
                }
            }
            nc.insert(condition.category, new_range);
            score += count_combinations(&workflows, &mut nc, &rule.dest);

            new_range = r.clone();

            match condition.operation {
                Operation::Greater => {
                    new_range.end = condition.value as i32;
                }
                Operation::Lesser => {
                    new_range.start = condition.value as i32;
                }
            }

            nc.insert(condition.category, new_range);
        } else {
            score += count_combinations(workflows, &mut nc, &rule.dest);
        }
    }
    score
}

fn cal_score(combination: &HashMap<Category, Range<i32>>) -> u64 {
    combination.iter().fold(1, |acc, (_, range)| {
        let mut r = range.clone();
        r.end += 1;

        acc * (r.len() as u64)
    })
}
