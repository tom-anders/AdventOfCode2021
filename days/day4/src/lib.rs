#![allow(unused_imports)]
use std::{slice::Chunks, collections::HashSet};
use std::str::FromStr;

use ndarray::ArrayView;
use utils::{Input, Solution};

type Matrix = ndarray::Array2<i64>;

#[derive(Clone)]
struct Board {
    mat: Matrix,
    marked: HashSet<i64>,
}

impl Board {
    pub fn new(lines: &[&String]) -> Board {
        let mut mat = Matrix::zeros((5, 5));
        lines.iter()
            .enumerate()
            .for_each(|(i, l)| {
                l.split_whitespace().map(|n| n.parse::<i64>().unwrap())
                    .enumerate()
                    .for_each(|(j, n)| {
                        mat[(i, j)] = n;
                    })
            });
        Board {
            mat,
            marked: HashSet::new(),
        }
    }

    pub fn mark_number(&mut self, num: &i64) {
        for n in self.mat.iter() {
            if n == num {
                self.marked.insert(*n);
            }
        }
    }

    pub fn is_winning(&self) -> bool {
        self.mat.rows().into_iter().any(|row| row.iter().all(|n| self.marked.contains(&n)))
        || self.mat.columns().into_iter().any(|col| col.iter().all(|n| self.marked.contains(&n)))
    }

    pub fn score(&self) -> i64 {
        self.mat.iter().filter(|n| !self.marked.contains(n)).sum()
    }
}

fn part1(numbers: &[i64], mut boards: Vec<Board>) -> i64 {
    for n in numbers.iter() {
        boards.iter_mut().for_each(|board| board.mark_number(n));

        let winning_board = boards.iter().filter(|board| board.is_winning()).next();
        if winning_board.is_some() {
            return winning_board.unwrap().score() * n;
        }
    }
    return 0;
}

fn part2(numbers: &[i64], mut boards: Vec<Board>) -> i64 {
    for n in numbers.iter() {
        boards.iter_mut().for_each(|board| board.mark_number(n));

        if boards.len() == 1 && boards.first().unwrap().is_winning() {
            return boards.first().unwrap().score() * n;
        }

        boards.retain(|board| !board.is_winning());
    }
    return 0;
}

pub fn solve(input: Input) -> Solution {
    let numbers: Vec<i64> = input
        .get_line(0)
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let boards: Vec<Board> = input
        .lines()
        .iter()
        .skip(2)
        .collect::<Vec<&String>>()
        .chunks(6)
        .map(|b| Board::new(b))
        .collect();

    Solution::new(part1(&numbers, boards.clone()), part2(&numbers, boards.clone()))
}
