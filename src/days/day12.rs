use pathfinding::prelude::bfs;
use std::collections::HashMap;
use std::fmt;
use std::time::Instant;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Coordinate {
    pub row: usize,
    pub col: usize,
}

impl Coordinate {
    fn from(row: usize, col: usize) -> Coordinate {
        Coordinate { row, col }
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

pub struct Graph {
    pub grid: HashMap<Coordinate, char>,
    pub start_loc: Option<Coordinate>,
    pub end_loc: Option<Coordinate>,
    pub extents: (usize, usize),
}

impl Graph {
    pub fn from(s: &str) -> Graph {
        let mut grid: HashMap<Coordinate, char> = HashMap::<Coordinate, char>::new();
        let mut start_loc: Option<Coordinate> = None;
        let mut end_loc: Option<Coordinate> = None;
        let mut row: usize = 0;
        let mut col: usize = 0;
        s.lines().for_each(|line| {
            for c in line.chars() {
                let coord = Coordinate { row, col };
                match c {
                    'S' => {
                        start_loc = Some(coord);
                        grid.insert(coord, 'a');
                    }
                    'E' => {
                        end_loc = Some(coord);
                        grid.insert(coord, 'z');
                    }
                    'a'..='z' => {
                        grid.insert(coord, c);
                    }
                    _ => {
                        panic!("main(): unexpected char in input!");
                    }
                }
                col += 1;
            }
            row += 1;
            col = 0;
        });
        let max_row: usize = grid.keys().map(|c| c.row).max().unwrap();
        let max_col: usize = grid.keys().map(|c| c.col).max().unwrap();
        let extents: (usize, usize) = (max_row, max_col);
        Graph {
            grid,
            start_loc,
            end_loc,
            extents,
        }
    }

    pub fn vertices(&self) -> Vec<Coordinate> {
        let mut vertices: Vec<Coordinate> = self.grid.keys().cloned().collect();
        vertices.sort();
        vertices
    }

    pub fn elevation(c: char) -> i32 {
        let elev: HashMap<char, i32> = ('a'..='z')
            .collect::<Vec<char>>()
            .into_iter()
            .zip((1..=26).collect::<Vec<i32>>().into_iter())
            .collect();
        elev[&c]
    }

    pub fn neighbors(&self, loc: Coordinate) -> Vec<Coordinate> {
        let mut adjacents: Vec<Coordinate> = vec![];
        let row = loc.row;
        let col = loc.col;
        if ((row as i32) - 1) >= 0 {
            adjacents.push(Coordinate::from(row - 1, col));
        }
        if (row + 1) <= self.extents.0 {
            adjacents.push(Coordinate::from(row + 1, col));
        }
        if ((col as i32) - 1) >= 0 {
            adjacents.push(Coordinate::from(row, col - 1));
        }
        if (col + 1) <= self.extents.1 {
            adjacents.push(Coordinate::from(row, col + 1));
        }
        let valid_neighbors: Vec<Coordinate> = adjacents
            .into_iter()
            .filter(|x| Graph::elevation(self.grid[x]) <= Graph::elevation(self.grid[&loc]) + 1)
            .collect();
        valid_neighbors
    }
}

pub fn part_1(payload: &str) -> usize {
    let graph: Graph = Graph::from(payload);
    bfs(
        &(graph.start_loc.unwrap()),
        |v| graph.neighbors(*v),
        |v| v == &(graph.end_loc.unwrap()),
    )
    .unwrap()
    .len()
        - 1
}

pub fn part_2(payload: &str) -> usize {
    let mut lengths: Vec<_> = vec![];
    let graph: Graph = Graph::from(payload);
    graph
        .vertices()
        .into_iter()
        .filter(|x| Graph::elevation(graph.grid[x]) == 1)
        .for_each(|start_node| {
            let result = bfs(
                &start_node,
                |v| graph.neighbors(*v),
                |v| v == &(graph.end_loc.unwrap()),
            );
            match result {
                None => { /* destination unreachable from this start */ }
                Some(path) => {
                    lengths.push(path.len() - 1);
                }
            }
        });
    lengths.into_iter().min().unwrap()
}

pub fn run(payload: &str) {
    let mut before = Instant::now();
    let steps = part_1(payload);
    println!(
        "Day 12 - Part 1 (elapsed time: {:.2?}): {}",
        before.elapsed(),
        steps
    );

    before = Instant::now();
    let steps_2 = part_2(payload);
    println!(
        "Day 12 - Part 2 (elapsed time: {:.2?}): {}",
        before.elapsed(),
        steps_2
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("samples/12.txt");

    #[test]
    fn find_least_steps() {
        assert_eq!(part_1(SAMPLE), 31)
    }

    #[test]
    fn hiking_exercise() {
        assert_eq!(part_2(SAMPLE), 29)
    }
}
