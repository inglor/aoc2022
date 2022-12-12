use regex::Regex;
use std::collections::BinaryHeap;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Clone)]
struct Monkey {
    id: usize,
    items: Vec<u64>,
    operation: Operation,
    divisible_by: u64,
    if_true: usize,
    if_false: usize,
    inspections: u64,
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Operation {
    Multiply(u64),
    Add(u64),
    Square,
}

impl FromStr for Monkey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^Monkey (?P<id>\d+):\n\s+Starting items: (?P<items>[\d,\s]*)\n\s+Operation: new = old (?P<operation>\S+)\s(?P<operation_arg>\S+)\n\s+Test: divisible by (?P<divisible_by>\d+)\n\s+\S+ true: throw to monkey (?P<if_true>\d+)\n\s+\S+ false: throw to monkey (?P<if_false>\d+)$").unwrap();
        let captures = re
            .captures(s)
            .ok_or(format!("Invalid input format for {}", s))?;

        let id = captures.name("id").unwrap().as_str().parse().unwrap();
        let items = captures
            .name("items")
            .unwrap()
            .as_str()
            .split(", ")
            .map(|s| s.parse())
            .collect::<Result<Vec<u64>, _>>()
            .map_err(|e| e.to_string())?;
        let operation = match captures.name("operation").unwrap().as_str() {
            "+" => Operation::Add(
                captures
                    .name("operation_arg")
                    .unwrap()
                    .as_str()
                    .parse()
                    .unwrap(),
            ),
            "*" => {
                if captures.name("operation_arg").unwrap().as_str().eq("old") {
                    Operation::Square
                } else {
                    Operation::Multiply(captures[4].parse().unwrap())
                }
            }
            _ => Operation::Square,
        };
        let divisible_by = captures
            .name("divisible_by")
            .unwrap()
            .as_str()
            .parse()
            .unwrap();
        let if_true = captures.name("if_true").unwrap().as_str().parse().unwrap();
        let if_false = captures.name("if_false").unwrap().as_str().parse().unwrap();

        Ok(Monkey {
            id,
            items,
            operation,
            divisible_by,
            if_true,
            if_false,
            inspections: 0,
        })
    }
}

fn calc_monkey_business(
    monkeys: &mut Vec<Monkey>,
    rounds: usize,
    calm_func: impl Fn(u64) -> u64,
) -> u64 {
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            for j in 0..monkeys[i].items.len() {
                let worry = match monkeys[i].operation {
                    Operation::Add(n) => calm_func(monkeys[i].items[j] + n),
                    Operation::Multiply(n) => calm_func(monkeys[i].items[j] * n),
                    Operation::Square => calm_func(monkeys[i].items[j] * monkeys[i].items[j]),
                };
                let recipient = if worry % monkeys[i].divisible_by == 0 {
                    monkeys[i].if_true
                } else {
                    monkeys[i].if_false
                };
                monkeys[recipient].items.push(worry);
            }
            monkeys[i].inspections += monkeys[i].items.len() as u64;
            monkeys[i].items.clear();
        }
    }

    let inspections: BinaryHeap<u64> = monkeys.iter().map(|m| m.inspections).collect();
    inspections.iter().take(2).product()
}

pub fn run(payload: &str) {
    let mut monkeys: Vec<Monkey> = payload
        .split("\n\n")
        .map(|monkey_def| Monkey::from_str(monkey_def.to_string().trim()).unwrap())
        .collect();
    let product: u64 = monkeys.iter().map(|m| m.divisible_by).product();
    println!(
        "Day 11 - Part 1: {}",
        calc_monkey_business(&mut monkeys.clone(), 20, |x| x / 3)
    );
    println!(
        "Day 11 - Part 2: {}",
        calc_monkey_business(&mut monkeys, 10_000, |x| x % product)
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;

    const SAMPLE: &str = include_str!("samples/11.txt");

    lazy_static! {
        static ref MONKEYS: Vec<Monkey> = SAMPLE
            .split("\n\n")
            .map(|monkey_def| Monkey::from_str(monkey_def.to_string().trim()).unwrap())
            .collect();
    }

    #[test]
    fn calm_div_3() {
        let mut monkeys = MONKEYS.to_vec();
        assert_eq!(calc_monkey_business(&mut monkeys, 20, |x| x / 3), 10605)
    }

    #[test]
    fn calm_product() {
        let mut monkeys = MONKEYS.to_vec();
        let product: u64 = monkeys.iter().map(|m| m.divisible_by).product();
        assert_eq!(
            calc_monkey_business(&mut monkeys, 10_000, |x| x % product),
            2713310158
        )
    }
}
