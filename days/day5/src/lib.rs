#![allow(unused_imports)]
use parse_display::{Display, FromStr};
use std::{cmp::{min, max}, collections::HashMap, mem::swap, ops::RangeInclusive};

use utils::{Solution, Input};

#[derive(Display, FromStr)]
#[display("{x0},{y0} -> {x1},{y1}")]
struct Line {
    x0: i64,
    y0: i64,
    x1: i64,
    y1: i64,
}

impl Line {
    pub fn is_diagonal(&self) -> bool {
        self.x0 != self.x1 && self.y0 != self.y1
    }

    pub fn points(&self) -> Vec<(i64, i64)> {
        if self.x0 == self.x1 { // horizontal
            (min(self.y0, self.y1) .. max(self.y0, self.y1) + 1).map(|y| (self.x0, y)).collect()
        } else if self.y0 == self.y1 {  // vertical
            (min(self.x0, self.x1) .. max(self.x0, self.x1) + 1).map(|x| (x, self.y0)).collect()
        } else { // Diagonal
            // Could probably write this part more elegantly, but this works...
            let inc_x = if self.x0 < self.x1 { 1 } else { -1 };
            let inc_y = if self.y0 < self.y1 { 1 } else { -1 };

            let mut pts = Vec::new();
            let mut x = self.x0;
            let mut y = self.y0;
            loop {
                pts.push((x, y));
                x += inc_x;
                y += inc_y;

                if x - inc_x == self.x1 { break; }
            }

            pts
        }
    }
}

#[cfg(test)]
mod test_line {
    use super::*;
    #[test]
    fn test_points() {
        assert_eq!("1,1 -> 1,3".parse::<Line>().unwrap().points(), vec![(1,1), (1,2), (1,3)]);
        assert_eq!("1,3 -> 1,1".parse::<Line>().unwrap().points(), vec![(1,1), (1,2), (1,3)]);
        assert_eq!("9,7 -> 7,7".parse::<Line>().unwrap().points(), vec![(7,7), (8,7), (9,7)]);
        assert_eq!("7,7 -> 9,7".parse::<Line>().unwrap().points(), vec![(7,7), (8,7), (9,7)]);

        assert_eq!("1,1 -> 3,3".parse::<Line>().unwrap().points(), vec![(1,1), (2,2), (3,3)]);
        assert_eq!("3,3 -> 1,1".parse::<Line>().unwrap().points(), vec![(3,3), (2,2), (1,1)]);
        assert_eq!("9,7 -> 7,9".parse::<Line>().unwrap().points(), vec![(9,7), (8,8), (7,9)]);
        assert_eq!("7,9 -> 9,7".parse::<Line>().unwrap().points(), vec![(7,9), (8,8), (9,7)]);
    }
}


fn count_overlap(lines: &[Line], skip_diag: bool) -> usize {
    let mut map = HashMap::new();
    for line in lines.iter().filter(|l| !skip_diag || !l.is_diagonal()) {
        for pt in line.points() {
            if !map.contains_key(&pt) {
                map.insert(pt, 1);
            } else {
                *map.get_mut(&pt).unwrap() += 1;
            }
        }
    }
    map.iter().filter(|(_, count)| *count >= &2).count()
}

pub fn solve(input: Input) -> Solution {
    let lines: Vec<Line> = input.parse();

    Solution::new(count_overlap(&lines, true), count_overlap(&lines, false))
}
