#![allow(unused_imports)]
use utils::{Input, Solution, ToDecimal};

struct Grid {
    rows: usize,
    cols: usize,
    data: Vec<Vec<usize>>,
}

impl Grid {
    pub fn new(input: &Input, padding: usize) -> Grid {
        let mut grid: Vec<Vec<_>> = input
            .raw
            .lines()
            .skip(2)
            .map(|s| s.chars().map(|c| if c == '#' { 1 } else { 0 }).collect())
            .collect();

        let rows = grid.len();
        let cols = grid[0].len();

        for v in &mut grid {
            for _ in 0..padding {
                v.insert(0, 0);
                v.push(0);
            }
        }

        for _ in 0..padding {
            grid.insert(0, vec![0; cols + 2 * padding]);
            grid.push(vec![0; cols + 2 * padding]);
        }

        Grid {
            rows: grid.len(),
            cols: grid[0].len(),
            data: grid,
        }
    }

    fn neighbor_indices(&self, i: usize, j: usize) -> Vec<(i32, i32)> {
        let (i, j) = (i as i32, j as i32);
        return vec![
            // Order is important here, need to scan row by row
            (i - 1, j - 1),
            (i - 1, j),
            (i - 1, j + 1),
            (i, j - 1),
            (i, j),
            (i, j + 1),
            (i + 1, j - 1),
            (i + 1, j),
            (i + 1, j + 1),
        ];
    }

    pub fn transform(&mut self, algo: &[usize]) {
        let mut new_data = self.data.clone();
        for i in 0..self.rows {
            for j in 0..self.cols {
                let val = self
                    .neighbor_indices(i, j)
                    .iter()
                    .map(|(row, col)| {
                        if *row >= 0
                            && *col >= 0
                            && *row < self.rows as i32
                            && *col < self.cols as i32
                        {
                            self.data[*row as usize][*col as usize]
                        } else {
                            self.data[i as usize][j as usize] // Boundary -> use same value as current
                        }
                    })
                    .collect::<Vec<_>>()
                    .to_decimal();

                new_data[i][j] = algo[val];
            }
        }
        self.data = new_data;
    }

    pub fn print(&self) {
        println!("Rows: {} Cols: {}", self.rows, self.cols);
        for v in &self.data {
            for i in v {
                print!("{}", if *i == 1 { '#' } else { '.' });
            }
            println!();
        }
    }

    pub fn count_lit(&self) -> usize {
        self.data
            .iter()
            .map(|row| row.iter().sum::<usize>())
            .sum::<usize>()
    }
}

pub fn solve(input: Input) -> Solution {
    let algo: Vec<_> = input
        .lines()
        .first()
        .unwrap()
        .chars()
        .map(|c| if c == '#' { 1 } else { 0 })
        .collect();

    let mut grid = Grid::new(&input, 50);

    let mut part1 = 0;
    for i in 0..50 {
        if i == 2 {
            part1 = grid.count_lit();
        }
        grid.transform(&algo);
    }
    let part2 = grid.count_lit();

    grid.print();

    Solution::new(part1, part2)
}
