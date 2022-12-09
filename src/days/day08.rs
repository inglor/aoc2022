use std::cmp::Ordering;
use grid::Grid;

enum View {
    Left,
    Right,
    Up,
    Down
}
struct TreePatch {
    trees: Grid<u32>,
}

impl TreePatch {
    pub fn new(payload: &str) -> Self {
        let mut trees: Grid<u32> = Grid::new(0, 0);
        payload
            .lines()
            .map(|line| line.chars().map(|digit| digit.to_digit(10).unwrap()).collect())
            .for_each(|row| trees.push_row(row));
        Self {
            trees
        }
    }

    pub fn count_visible_trees(&self) -> usize {
        let mut visible_counter: usize = (self.trees.rows() + self.trees.cols()) * 2 - 4;

        for row in 1..self.trees.rows() - 1 {
            for column in 1..self.trees.cols() - 1 {
                if self.get_directions(row, column)
                    .iter()
                    .map(|sequence| sequence.1.iter().any(|&x| x >= self.trees[row][column]))
                    .any(|invisible| !invisible)
                {
                    visible_counter += 1
                }
            }
        }
        visible_counter
    }

    pub fn scenic_scores(&self) -> Vec<usize> {
        let mut scenic_scores: Vec<usize> = vec!();

        for row in 0..self.trees.rows() {
            for column in 0..self.trees.cols() {

                let target = &self.trees[row][column];

                let test: Vec<usize> = self.get_directions(row, column)
                    .iter()
                    .map(|val| {
                        let mut counter = 0;
                        let (direction, sequence) = val;
                        let mut numbers = sequence.clone();

                        match direction {
                            View::Left | View::Up => { numbers = sequence.iter().rev().copied().collect(); }
                            _ => ()
                        }

                        for nr in &numbers {
                            match nr.cmp(target) {
                                Ordering::Less => { counter += 1 }
                                Ordering::Equal | Ordering::Greater => { counter += 1; break }
                            }
                        }
                        counter
                    })
                    .collect();

                scenic_scores.push(test.iter().product());
            }
        }
        scenic_scores
    }
    fn get_directions (&self, row: usize, pos: usize) -> Vec<(View, Vec<u32>)> {
        vec![(View::Left, self.trees[row][..pos].to_vec()),
             (View::Right, self.trees[row][pos + 1..].to_vec()),
             (View::Up, self.trees.iter_col(pos).take(row).cloned().collect()),
             (View::Down, self.trees.iter_col(pos).skip(row + 1).cloned().collect())]
    }
}

fn part1(payload: &str) {
    println!("{}", TreePatch::new(payload).count_visible_trees())
}

fn part2(payload: &str) {
    println!("{}", *TreePatch::new(payload).scenic_scores().iter().max().unwrap())
}

pub fn run(payload: &str) {
    part1(payload);
    part2(payload);
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = concat!("30373\n", "25512\n", "65332\n", "33549\n", "35390\n");

    #[test]
    fn count_visible_trees() {
        assert_eq!(TreePatch::new(EXAMPLE).count_visible_trees(), 21)
    }

    #[test]
    fn max_scenic_scores() {
        assert_eq!(*TreePatch::new(EXAMPLE).scenic_scores().iter().max().unwrap(), 8)
    }
}
