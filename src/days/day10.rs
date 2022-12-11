use itertools::Itertools;

const WIDTH: usize = 40;
const HEIGHT: usize = 6;

struct Cpu {
    x: i32,
    cycle: i32,
    signal_strength: i32,
    screen: [[bool; WIDTH]; HEIGHT],
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            x: 1,
            cycle: 0,
            signal_strength: 0,
            screen: [[false; WIDTH]; HEIGHT],
        }
    }

    pub fn print_screen(&self) -> String {
        self.screen
            .iter()
            .map(|row| row.iter().map(|&x| if x { "#" } else { "." }).join(""))
            .join("\n")
    }

    fn tick(&mut self) {
        let y = (self.cycle / WIDTH as i32) as usize;
        let x = (self.cycle % WIDTH as i32) as usize;
        self.screen[y][x] = self.x.abs_diff(x as i32) <= 1;
        self.cycle += 1;

        if self.cycle % 40 == 20 {
            self.signal_strength += self.x * self.cycle;
        }
    }

    pub fn noop(&mut self) {
        self.tick();
    }

    pub fn addx(&mut self, v: i32) {
        self.tick();
        self.tick();
        self.x += v;
    }
}

pub fn run(payload: &str) {
    let mut cpu = Cpu::new();
    payload.lines().for_each(|line| match &line[0..4] {
        "noop" => cpu.noop(),
        "addx" => cpu.addx(line[5..].parse().unwrap()),
        _ => panic!("Invalid command {}", line),
    });

    println!("Day 10 - Part 1: {}", cpu.signal_strength);
    println!("Day 10 - Part 2:\n {}", cpu.print_screen());
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;

    const SAMPLE: &str = include_str!("samples/10.txt");

    lazy_static! {
        static ref EXAMPLE_CPU: Cpu = {
            let mut _cpu = Cpu::new();
            SAMPLE.lines().for_each(|line| match &line[0..4] {
                "noop" => _cpu.noop(),
                "addx" => _cpu.addx(line[5..].parse().unwrap()),
                _ => panic!("Invalid command"),
            });
            _cpu
        };
    }

    #[test]
    fn cpu_signal() {
        assert_eq!(EXAMPLE_CPU.signal_strength, 13140)
    }

    #[test]
    fn cpu_screen() {
        assert_eq!(
            concat!(
                "##..##..##..##..##..##..##..##..##..##..\n",
                "###...###...###...###...###...###...###.\n",
                "####....####....####....####....####....\n",
                "#####.....#####.....#####.....#####.....\n",
                "######......######......######......####\n",
                "#######.......#######.......#######....."
            ),
            EXAMPLE_CPU.print_screen()
        );
    }
}
