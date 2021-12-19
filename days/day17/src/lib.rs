#![allow(unused_imports)]
use std::collections::HashSet;

use parse_display::{Display, FromStr};
use utils::{Input, Solution};

#[derive(FromStr, Display, Debug)]
#[display("target area: x={x1}..{x2}, y={y1}..{y2}\n")]
struct Target {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
}

impl Target {
    pub fn x_l(&self) -> i32 { self.x1.min(self.x2) }
    pub fn x_r(&self) -> i32 { self.x1.max(self.x2) }
    pub fn y_u(&self) -> i32 { self.y1.max(self.y2) }
    pub fn y_d(&self) -> i32 { self.y1.min(self.y1) }

    pub fn inside_target(&self, x: &i32, y: &i32) -> bool {
           *x >= self.x_l() && *x <= self.x_r()
        && *y <= self.y_u() && *y >= self.y_d()
    }

    pub fn over_target(&self, x: &i32, y: &i32) -> bool {
        *x > self.x_r() || *y < self.y_d()
    }
}

struct Simulator {
    target: Target,
    v0x: i32,
    v0y: i32,
}

impl Simulator {
    pub fn y(&self, t: i32) -> i32 {
        t * self.v0y - (t * t - t) / 2
    }

    pub fn max_y(&self) -> i32 {
        self.y(self.v0y)
    }

    pub fn x(&self, t: i32) -> i32 {
        if t > self.v0x {
            self.x(self.v0x)
        } else {
            t * self.v0x - (t * t - t) / 2
        }
    }

    

    pub fn hits_target(&self) -> Option<i32> {
        let mut t = 0;

        loop {
            let x = self.x(t);
            let y = self.y(t);
            if self.target.inside_target(&x, &y) {
                return Some(self.max_y());
            } else if self.target.over_target(&x, &y) {
                return None;
            }

            t += 1;
        }
    }
}

pub fn solve(input: Input) -> Solution {
    let simulator = Simulator {
        target: input.raw.parse().unwrap(),
        v0x: 6,
        v0y: 9,
    };

    let mut max = 0;

    let min_vx = ((2.0 * simulator.target.x_l() as f64 - 0.25).sqrt() - 0.5).ceil() as i32;
    let max_vx = simulator.target.x_r() + 1;

    // Could probably improve this even more by making it a function of v0x
    let min_vy = simulator.target.y_d();

    let mut count = 0;

    for v0x in min_vx..max_vx {
        // 500 is a heuristic, couldn't figure out an upper bound here yet
        for v0y in min_vy..500 { 
            let simulator = Simulator {
                target: input.raw.parse().unwrap(),
                v0x,
                v0y,
            };
            if let Some(max_y) = simulator.hits_target() {
                if max_y > max {
                    max = max_y;
                }
                count += 1;
            };
        }
    }

    Solution::new(max, count)
}
