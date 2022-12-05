use itertools::Itertools;

#[derive(PartialEq, Eq, Clone)]
struct Camp {
    positions: Vec<Vec<Crate>>,
    version: u16,
}

#[derive(PartialEq, Eq, Clone)]
struct Crate {
    id: char,
}

impl Camp {
    fn new(positions: Vec<Vec<Crate>>, version: u16) -> Camp {
        Camp { positions, version }
    }

    fn move_crates(&mut self, crate_count: usize, from: usize, to: usize) {
        let from = &mut self.positions[from - 1];
        let crates = from.split_off(from.len() - crate_count);
        if self.version == 9000 {
            self.positions[to - 1].extend(crates.iter().cloned().rev());
        } else {
            self.positions[to - 1].extend(crates.iter().cloned());
        }
    }

    fn get_top_row(&self) -> String {
        self.positions
            .iter()
            .filter_map(|m| m.last().map(|c| c.id))
            .join("")
    }
}

fn simulate(payload: &str, version: u16) -> Camp {
    let (init, moves) = payload.split_once("\n\n").unwrap();
    let mut stack_iter = init.lines().rev();
    let mut stack = vec![vec![]; stack_iter.next().unwrap().len() / 4 + 1];

    stack_iter.for_each(|l| {
        l.chars().skip(1).enumerate().for_each(|(i, c)| {
            if i % 4 == 0 && c != ' ' {
                stack[i / 4].push(Crate { id: c });
            }
        });
    });

    let mut camp = Camp::new(stack, version);

    moves
        .lines()
        .map(|l| {
            let s: Vec<&str> = l.split_ascii_whitespace().collect();
            (
                s[1].parse().unwrap(),
                s[3].parse().unwrap(),
                s[5].parse().unwrap(),
            )
        })
        .for_each(|(x, y, z)| camp.move_crates(x, y, z));
    camp
}

fn part1(payload: &str) {
    println!("{}", simulate(payload, 9000).get_top_row());
}

fn part2(payload: &str) {
    println!("{}", simulate(payload, 9001).get_top_row());
}

pub fn run(payload: &str) {
    part1(payload);
    part2(payload);
}

#[cfg(test)]
mod tests {
    use super::*;
    static EXAMPLE: &str = concat!(
        "    [D]    \n",
        "[N] [C]\n",
        "[Z] [M] [P]\n",
        " 1   2   3 \n",
        "\n",
        "move 1 from 2 to 1\n",
        "move 3 from 1 to 3\n",
        "move 2 from 2 to 1\n",
        "move 1 from 1 to 2\n"
    );

    #[test]
    fn simulate_9000() {
        assert_eq!(simulate(EXAMPLE, 9000).get_top_row(), "CMZ")
    }
    #[test]
    fn simulate_9001() {
        assert_eq!(simulate(EXAMPLE, 9001).get_top_row(), "MCD")
    }
}
