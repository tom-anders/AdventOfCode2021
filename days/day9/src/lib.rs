#![allow(unused_imports)]
use std::{collections::HashSet, iter::FromIterator};

use utils::{Input, Solution};

struct Grid {
    rows: usize,
    cols: usize,
    data: Vec<usize>,
}

impl Grid {
    pub fn new(grid: Vec<Vec<usize>>) -> Grid {
        Grid {
            rows: grid.len(),
            cols: grid[0].len(),
            data: grid.into_iter().flatten().collect(),
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, usize> {
        self.data.iter()
    }

    fn to_1d_index(&self, index: &(usize, usize)) -> usize {
        index.0 * self.cols + index.1 // Row major
    }

    pub fn at(&self, index: &(usize, usize)) -> usize {
        self.data[self.to_1d_index(index)]
    }

    fn neighbor_indices(&self, index: (usize, usize)) -> Vec<(usize, usize)> {
        let (i, j) = index;
        let mut nb = Vec::new();
        if i > 0 {
            nb.push((i - 1, j));
        }
        if i < self.rows - 1 {
            nb.push((i + 1, j));
        }

        if j > 0 {
            nb.push((i, j - 1));
        }
        if j < self.cols - 1 {
            nb.push((i, j + 1));
        }

        nb
    }

    fn neighbors(&self, index: (usize, usize)) -> Vec<usize> {
        self.neighbor_indices(index)
            .iter()
            .map(|(x, y)| self.at(&(*x, *y)))
            .collect()
    }

    fn make_index(&self, index: &usize) -> (usize, usize) {
        (index / self.cols, index % self.cols)
    }

    fn find_low_point(&self, index: usize) -> Option<usize> {
        let mut pt = (index, self.data[index]);
        loop {
            let neigh = self.neighbor_indices(self.make_index(&pt.0));
            let lower: Vec<_> = neigh.iter().filter(|i| pt.1 > self.at(*i)).collect();

            if lower.is_empty() {
                return Some(pt.0);
            }
            if lower.len() > 1 {
                return None;
            }
            pt.0 = self.to_1d_index(lower[0]);
            pt.1 = self.at(lower[0]);
        }
    }

    fn basin_size(&self, low_point_index: usize) -> usize {
        let mut points = HashSet::new();
        points.insert(self.make_index(&low_point_index));

        let mut new_points = vec![self.make_index(&low_point_index)];
        loop {
            new_points = new_points.iter()
                .map(|index| {
                    self.neighbor_indices(*index)
                        .iter()
                        .filter(|index| self.at(index) != 9 && !points.contains(index))
                        .copied()
                        .collect::<Vec<_>>()
                }).flatten().collect();

            for p in &new_points {
                points.insert(*p);
            }

            if new_points.is_empty() {
                return points.len();
            }
        }
    }
}

pub fn solve(input: Input) -> Solution {
    let grid = Grid::new(
        input
            .parse::<String>()
            .iter()
            .map(|s| {
                s.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect(),
    );

    let low_points: Vec<_> = grid
        .iter()
        .enumerate()
        .filter(|(index, val)| {
            grid.neighbors(grid.make_index(index))
                .iter()
                .all(|n| val < &n)
        })
        .collect();

    let part1: usize = low_points.iter().map(|(_, val)| *val + 1).sum();

    let mut part2: Vec<_> = low_points.iter()
        .map(|(index, _)| grid.basin_size(*index))
        .collect();

    part2.sort();
    part2.reverse();

    Solution::new(part1, part2[0..3].iter().product::<usize>())
}
