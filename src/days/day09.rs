use std::collections::HashSet;

type Coord = (i32, i32);

struct Map {
    knots: [Coord; 10],
    visited_1: HashSet<Coord>,
    visited_9: HashSet<Coord>,
}

impl Map {
    pub fn new() -> Self {
        let knots: [Coord; 10] = [(0, 0); 10];
        let mut visited_1: HashSet<Coord> = HashSet::new();
        let mut visited_9: HashSet<Coord> = HashSet::new();
        visited_1.insert(knots[1]);
        visited_9.insert(knots[9]);
        Map {
            knots,
            visited_1,
            visited_9,
        }
    }

    pub fn simulate_steps(&mut self, payload: &str) -> &mut Self {
        for line in payload.lines() {
            let (dir, num_steps) = line.split_once(' ').unwrap();
            let num_steps: u32 = num_steps.parse().unwrap();
            let delta = match dir {
                "L" => (-1, 0),
                "R" => (1, 0),
                "U" => (0, -1),
                "D" => (0, 1),
                _ => unreachable!(),
            };

            for _ in 0..num_steps {
                self.knots[0] = (self.knots[0].0 + delta.0, self.knots[0].1 + delta.1);

                for i in 1..10 {
                    let dx = self.knots[i - 1].0 - self.knots[i].0;
                    let dy = self.knots[i - 1].1 - self.knots[i].1;

                    if dx.abs() > 1 || dy.abs() > 1 {
                        self.knots[i].0 += dx.signum();
                        self.knots[i].1 += dy.signum();
                    }
                }

                self.visited_1.insert(self.knots[1]);
                self.visited_9.insert(self.knots[9]);
            }
        }
        self
    }
}

fn part1(payload: &str) {
    println!(
        "Day 9 - Part 1: {}",
        Map::new().simulate_steps(payload).visited_1.len()
    )
}

fn part2(payload: &str) {
    println!(
        "Day 9 - Part 2: {}",
        Map::new().simulate_steps(payload).visited_9.len()
    )
}

pub fn run(payload: &str) {
    part1(payload);
    part2(payload);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str =
        concat!("R 4\n", "U 4\n", "L 3\n", "D 1\n", "R 4\n", "D 1\n", "L 5\n", "R 2\n");
    const EXAMPLE2: &str =
        concat!("R 5\n", "U 8\n", "L 8\n", "D 3\n", "R 17\n", "D 10\n", "L 25\n", "U 20");

    #[test]
    fn knots_2() {
        assert_eq!(Map::new().simulate_steps(EXAMPLE1).visited_1.len(), 13)
    }

    #[test]
    fn knots_10() {
        assert_eq!(Map::new().simulate_steps(EXAMPLE2).visited_9.len(), 36)
    }
}
