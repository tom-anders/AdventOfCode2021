#![allow(unused_imports)]
use std::{collections::HashSet, iter::FromIterator};

use parse_display::{Display, FromStr};
use utils::{Solution, Input};

#[derive(Display, FromStr, Debug, PartialEq, Eq, Hash)]
#[display("{x},{y}")]
struct Point {
    x: i32, 
    y: i32,
}

#[derive(Display, FromStr, Debug)]
enum Line {
    #[display("fold along x={0}")]
    X(i32),
    #[display("fold along y={0}")]
    Y(i32),
}

fn split(points: &HashSet<Point>, line: &Line) -> HashSet<Point> {
    let mut result = HashSet::new();
    match line {
        Line::Y(y) => {
            for pt in points.iter() {
                assert!(pt.y != *y);
                result.insert(Point{ x: pt.x, y: if pt.y > *y { pt.y - 2 * (pt.y - *y) } else { pt.y } });
            }
        },
        Line::X(x) => {
            for pt in points.iter() {
                assert!(pt.x != *x);
                result.insert(Point{ x: if pt.x > *x { pt.x - 2 * (pt.x - *x) } else { pt.x }, y: pt.y });
            }
        }
    } 

    result
}

pub fn solve(input: Input) -> Solution {
    let mut points: HashSet<Point> = input.raw.split("\n\n").next().unwrap().lines().map(|s| s.parse().unwrap()).collect();
    let lines: Vec<Line> = input.raw.split("\n\n").last().unwrap().lines().map(|s| s.parse().unwrap()).collect();

    let part1 = split(&points, lines.first().unwrap()).len();

    for l in &lines {
        points = split(&points, l);
    }

    let max_x = points.iter().max_by(|lhs, rhs| lhs.x.cmp(&rhs.x)).unwrap().x;
    let max_y = points.iter().max_by(|lhs, rhs| lhs.y.cmp(&rhs.y)).unwrap().y;

    for y in 0..max_y + 1 {
        for x in 0..max_x + 1 {
            print!("{}", if points.get(&Point{x, y}).is_some() { "#" } else { " " });
        }
        println!();
    }

    Solution::new(part1, "See above")
}
