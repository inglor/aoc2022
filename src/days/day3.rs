use std::collections::HashSet;

#[derive(PartialEq, Eq, Clone)]
struct RuckSack {
    items: Vec<char>,
    priority: i32,
}

impl RuckSack {
    fn new(rucksack: String) -> RuckSack {
        let items = rucksack.chars().collect();
        let mut first: Vec<char> = rucksack.chars().collect();
        let second: Vec<char> = first.split_off(first.len() / 2);
        let temp_data: HashSet<char> = HashSet::from_iter(first.iter().cloned());
        let temp_data2: HashSet<char> = HashSet::from_iter(second.iter().cloned());
        let common_elements: HashSet<char> = temp_data.intersection(&temp_data2).copied().collect();
        let priority = common_elements
            .iter()
            .map(|c| RuckSack::char_to_priority(*c))
            .reduce(|a, b| a + b)
            .unwrap_or(0);
        RuckSack { items, priority }
    }

    fn char_to_priority(c: char) -> i32 {
        let ascii_code = c as i32;
        // 65-90 = A-Z
        if (65..=90).contains(&ascii_code) {
            return ascii_code - 65 + 27;
        }
        // 97-122 = a-z
        ascii_code - 97 + 1
    }
}

fn part1(payload: &str) {
    let total_priority = payload
        .lines()
        .map(|l| RuckSack::new(String::from(l)))
        .map(|r| r.priority)
        .reduce(|a, b| a + b)
        .unwrap_or(0);
    println!("Part 1: {}", total_priority);
}

fn part2(payload: &str) {
    let rucksacks: Vec<RuckSack> = payload
        .lines()
        .map(|l| RuckSack::new(String::from(l)))
        .collect();

    let mut priority_sum = 0;
    for group_index in (0..rucksacks.len()).step_by(3) {
        let rucksack_1 = &rucksacks[group_index];
        let rucksack_2 = &rucksacks[group_index + 1];
        let rucksack_3 = &rucksacks[group_index + 2];

        for c in rucksack_1.items.iter() {
            if rucksack_2.items.contains(c) && rucksack_3.items.contains(c) {
                priority_sum += RuckSack::char_to_priority(*c);
                break;
            }
        }
    }
    println!("Part 2: {}", priority_sum);
}

pub fn run(payload: &str) {
    part1(payload);
    part2(payload);
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("ABCabcA", 27)]
    #[case("A", 0)]
    #[case("abba", 3)]
    #[case("abccba", 6)]
    #[case("ABAB", 55)]
    #[case("vJrwpWtwJgWrhcsFMMfFFhFp", 16)]
    #[case("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", 38)]
    #[case("PmmdzqPrVvPwwTWBwg", 42)]
    #[case("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn", 22)]
    #[case("ttgJtRGJQctTZtZT", 20)]
    #[case("CrZsJsPPZsGzwwsLwLmpwMDw", 19)]
    fn ruck_sack_priority(#[case] input: String, #[case] expected: i32) {
        let ruck_sack = RuckSack::new(input);
        assert_eq!(ruck_sack.priority, expected)
    }
}
