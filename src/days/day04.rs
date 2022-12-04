use std::ops::RangeInclusive;

#[derive(PartialEq, Eq, Clone)]
struct Assignments {
    parts: RangeInclusive<i32>,
}

impl Assignments {
    fn new(input: &str) -> Assignments {
        let (start, end) = input.split_once('-').unwrap();
        let option: RangeInclusive<i32> = start.parse().ok().unwrap()..=(end.parse().ok().unwrap());
        Assignments { parts: option }
    }

    fn envelops(&self, other: &Assignments) -> bool {
        other.parts.start() >= self.parts.start() && other.parts.end() <= self.parts.end()
    }

    fn count_overlaps(&self, other: &Assignments) -> usize {
        other
            .parts
            .clone()
            .into_iter()
            .filter(|x| self.parts.contains(x))
            .count()
    }
}

fn part1(payload: &str) {
    let count = payload
        .lines()
        .filter_map(|l| {
            let (a, b) = l.split_once(',')?;
            Some((a, b))
        })
        .map(|(a, b)| (Assignments::new(a), Assignments::new(b)))
        .filter(|(a, b)| a.envelops(b) || b.envelops(a))
        .count();
    println!("Part 1: {}", count);
}

fn part2(payload: &str) {
    let count = payload
        .lines()
        .filter_map(|l| {
            let (a, b) = l.split_once(',')?;
            Some((a, b))
        })
        .map(|(a, b)| (Assignments::new(a), Assignments::new(b)))
        .filter(|(a, b)| a.count_overlaps(b) > 0)
        .count();
    println!("Part 2: {}", count);
}

pub fn run(payload: &str) {
    part1(payload);
    part2(payload);
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("2-4", "6-8", false)] // 2-4,6-8
    #[case("2-3", "4-5", false)] // 2-3,4-5
    #[case("5-7", "7-9", false)] // 5-7,7-9
    #[case("2-8", "3-7", true)] // 2-8,3-7
    #[case("6-6", "4-6", true)] // 6-6,4-6
    #[case("2-6", "4-8", false)] // 2-6,4-8
    fn envelops(#[case] this: String, #[case] that: String, #[case] expected: bool) {
        assert_eq!(
            Assignments::new(this.as_str()).envelops(&Assignments::new(that.as_str()))
                || Assignments::new(that.as_str()).envelops(&Assignments::new(this.as_str())),
            expected
        )
    }

    #[rstest]
    #[case("2-4", "6-8", 0)] // 2-4,6-8
    #[case("2-3", "4-5", 0)] // 2-3,4-5
    #[case("5-7", "7-9", 1)] // 5-7,7-9
    #[case("2-8", "3-7", 5)] // 2-8,3-7
    #[case("6-6", "4-6", 1)] // 6-6,4-6
    #[case("2-6", "4-8", 3)] // 2-6,4-8
    fn overlaps(#[case] this: String, #[case] that: String, #[case] expected: usize) {
        assert_eq!(
            Assignments::new(this.as_str()).count_overlaps(&Assignments::new(that.as_str())),
            expected
        )
    }
}
