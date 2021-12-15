#![allow(unused_imports)]
use std::cmp::Reverse;
use utils::{Input, Solution};
use priority_queue::PriorityQueue;

#[derive(Clone, Copy)]
struct Node {
    cost: usize,
    distance: usize,
    visited: bool,
}

impl Node {
    fn new(cost: usize) -> Node {
        Node { cost, distance: usize::MAX, visited: false }
    }
}

struct Grid {
    rows: usize,
    cols: usize,
    data: Vec<Vec<Node>>,
}

impl Grid {
    pub fn new(s: &str) -> Grid {
        let grid: Vec<Vec<_>> = s
            .lines()
            .map(|s| {
                s.chars()
                    .map(|c| Node::new(c.to_digit(10).unwrap() as usize))
                    .collect()
            })
            .collect();
        Grid {
            rows: grid.len(),
            cols: grid[0].len(),
            data: grid,
        }
    }

    pub fn make_extended_grid(&self) -> Grid {
        let mut row = Vec::<Node>::new();
        row.resize_with(self.rows * 5, || Node::new(0));
        let mut data: Vec<Vec<Node>> = Vec::new();
        data.resize(self.cols * 5, row);

        for i in 0..self.rows * 5 {
            for j in 0..self.cols * 5 {
                let mut cost = self.data[i % self.rows][j % self.cols].cost + (i / self.rows) + (j / self.cols);

                if cost > 9 {
                    cost -= 9;
                }
                data[i][j] = Node::new(cost);
            }
        }

        Grid { rows: 5 * self.rows, cols: 5 * self.cols, data }
    }

    pub fn print(&self) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                print!("{} ", self.data[i][j].cost);
            }
            println!();
        }
        println!();
    }

    fn neighbor_indices(&self, index: (i32, i32)) -> Vec<(usize, usize)> {
        let (i, j) = index;

        [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)]
            .iter()
            .filter(|(i, j)| *i >= 0 && *j >= 0 && *i < self.rows as i32 && *j < self.cols as i32)
            .map(|(i, j)| (*i as usize, *j as usize))
            .collect()
    }

    // Using Dijkstra's algorithm
    pub fn find_exit(&mut self) -> usize {
        let mut queue = PriorityQueue::<(usize, usize), std::cmp::Reverse<usize>>::new();
        queue.push((0_usize, 0_usize), Reverse(0));
        self.data[0][0].distance = 0;

        loop {
            let ((row, col), _) = queue.pop().unwrap();

            self.data[row][col].visited = true;
            let current_distance = self.data[row][col].distance;

            let neighbors = self.neighbor_indices((row as i32, col as i32));
            for (i, j) in neighbors {
                let mut next = &mut self.data[i][j];
                if !next.visited {
                    next.distance = next.distance.min(next.cost + current_distance);

                    queue.push_increase((i, j), Reverse(next.distance));
                }
            }

            if row == self.rows - 1 && col == self.cols -1 {
                return self.data[row][col].distance;            
            }
        }
    }

}

pub fn solve(input: Input) -> Solution {
    let mut grid = Grid::new(&input.raw);

    let mut extended_grid = grid.make_extended_grid();

    Solution::new(grid.find_exit(), extended_grid.find_exit())
}
