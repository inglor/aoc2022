use itertools::Itertools;

struct SignalDevice {
    data_stream: String,
}

impl SignalDevice {
    pub fn new(data_stream: String) -> Self {
        Self { data_stream }
    }

    pub fn lock_signal(&self, marker: usize) -> usize {
        self.data_stream
            .chars()
            .collect::<Vec<char>>()
            .windows(marker)
            .find_position(|x| x.iter().all_unique())
            .map(|(x, _)| x + marker)
            .unwrap()
    }
}

fn part1(payload: String) {
    println!("{}", SignalDevice::new(payload).lock_signal(4));
}

fn part2(payload: String) {
    println!("{}", SignalDevice::new(payload).lock_signal(14));
}

pub fn run(payload: &str) {
    part1(payload.to_string());
    part2(payload.to_string());
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 4, 5)]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", 4, 6)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4, 10)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4, 11)]
    #[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14, 19)]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 14, 23)]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", 14, 23)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14, 29)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14, 26)]
    fn lock_signal(#[case] this: String, #[case] marker: usize, #[case] expected: usize) {
        assert_eq!(SignalDevice::new(this).lock_signal(marker), expected)
    }
}
