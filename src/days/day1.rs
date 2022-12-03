use itertools::Itertools;

fn read_calories(input_file: &str) -> Vec<i32> {
    input_file
        .lines()
        .map(|v| v.parse::<i32>().ok())
        .batching(|it| {
            let mut sum = None;
            while let Some(Some(v)) = it.next() {
                sum = Some(sum.unwrap_or(0) + v);
            }
            sum
        })
        .collect()
}

fn part1(input_file: &str) {
    let calories = read_calories(input_file);
    let answer = calories.iter().max().unwrap();
    println!("Part1: {answer:?}");
}

fn part2(input_file: &str) {
    let mut calories = read_calories(input_file);
    calories.sort_by(|a, b| b.cmp(a));
    let answer = calories.iter().take(3).sum::<i32>();
    println!("Part2: {answer:?}");
}

pub fn run() {
    let payload = include_str!("inputs/01.txt");
    part1(payload);
    part2(payload);
}
