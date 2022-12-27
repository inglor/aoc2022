use serde_json::{from_str, json, Value};
use std::cmp::Ordering;
use std::time::Instant;

#[derive(Debug, Eq, PartialEq)]
struct Pair {
    idx: usize,
    left: Value,
    right: Value,
}

impl Pair {
    pub fn from_str(idx: usize, s: &str) -> Option<Pair> {
        let mut lists = s.split('\n');
        if let Some(list1) = lists.next() {
            let list1: Value = from_str(list1).unwrap();
            if let Some(list2) = lists.next() {
                let list2: Value = from_str(list2).unwrap();
                return Some(Pair {
                    idx,
                    left: list1,
                    right: list2,
                });
            }
        }
        None
    }
}

fn compare(a: &Value, b: &Value) -> Option<Ordering> {
    match (a, b) {
        (Value::Number(a), Value::Number(b)) => match a.as_u64().cmp(&b.as_u64()) {
            Ordering::Equal => None,
            order => Some(order),
        },
        (Value::Array(a), Value::Array(b)) => {
            if a.is_empty() || b.is_empty() {
                match a.len().cmp(&b.len()) {
                    Ordering::Equal => None,
                    order => Some(order),
                }
            } else if let Some(v) = compare(&a[0], &b[0]) {
                Some(v)
            } else {
                compare(&json!(a[1..]), &json!(b[1..]))
            }
        }
        (Value::Number(a), Value::Array(b)) => compare(&json!(vec![a]), &json!(b)),
        (Value::Array(a), Value::Number(b)) => compare(&json!(a), &json!(vec![b])),
        _ => Some(Ordering::Greater),
    }
}

pub fn part_1(payload: &str) -> usize {
    let mut pairs: Vec<Pair> = vec![];
    for (idx, pair_def) in payload.split("\n\n").enumerate() {
        if let Some(p) = Pair::from_str(idx + 1, pair_def) {
            pairs.push(p);
        }
    }
    pairs
        .iter()
        .map(|p| compare(&p.left, &p.right))
        .enumerate()
        .filter(|(_, p)| p.is_some() && matches!(p.unwrap(), Ordering::Less))
        .map(|(i, _)| i + 1)
        .sum::<usize>()
}
pub fn part_2(payload: &str) -> usize {
    let mut packets: Vec<Value> = payload
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    packets.extend([json!([[2]]), json!([[6]])]);
    packets.sort_by(|a, b| compare(a, b).unwrap());

    let dp1 = packets.iter().position(|p| *p == json!([[2]])).unwrap() + 1;
    let dp2 = packets.iter().position(|p| *p == json!([[6]])).unwrap() + 1;
    dp1 * dp2
}

pub fn run(payload: &str) {
    let mut before = Instant::now();
    let steps = part_1(payload);
    println!(
        "Day 13 - Part 1 (elapsed time: {:.2?}): {}",
        before.elapsed(),
        steps
    );

    before = Instant::now();
    let steps_2 = part_2(payload);
    println!(
        "Day 13 - Part 2 (elapsed time: {:.2?}): {}",
        before.elapsed(),
        steps_2
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("samples/13.txt");

    #[test]
    fn part_one() {
        assert_eq!(part_1(SAMPLE), 13)
    }

    #[test]
    fn part_two() {
        assert_eq!(part_2(SAMPLE), 140)
    }
}
