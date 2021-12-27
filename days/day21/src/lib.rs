#![allow(unused_imports)]
use std::collections::HashMap;

use parse_display::{Display, FromStr};
use utils::{Input, Solution};

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Player {
    pos: usize,
    score: usize,
}

impl Player {
    pub fn new(pos: usize) -> Player {
        Player { pos, score: 0 }
    }

    pub fn move_forward(&mut self, amount: &usize) {
        self.pos += amount;
        while self.pos > 10 {
            self.pos -= 10;
        }
        self.score += self.pos;
    }
}

struct DetDie {
    next: usize,
    rolls: usize,
}

impl DetDie {
    pub fn new() -> DetDie {
        DetDie { next: 0, rolls: 0 }
    }

    pub fn roll(&mut self) -> usize {
        let score = 3 * self.next + 6;

        self.next += 3;
        if self.next > 100 {
            self.next -= 100;
        }

        self.rolls += 3;

        score
    }
}

fn part1() -> usize {
    let mut players = vec![Player::new(6), Player::new(9)];

    let mut die = DetDie::new();

    let mut player_index = 0;

    loop {
        players[player_index].move_forward(&die.roll());

        if players[player_index].score > 1000 {
            return die.rolls * players[(player_index + 1) % 2].score;
        }

        player_index = (player_index + 1) % 2;
    }
}

fn gen_possible_rolls() -> HashMap<usize, usize> {
    let mut map = HashMap::new();
    for i in 1..4 {
        for j in 1..4 {
            for k in 1..4 {
                let roll = i + j + k;
                if map.contains_key(&roll) {
                    *map.get_mut(&roll).unwrap() += 1;
                } else {
                    map.insert(roll, 1);
                }
            }
        }
    }
    map
}

fn part2() -> usize {
    let mut universes = HashMap::new();

    //TODO use my input here
    universes.insert((Player::new(6), Player::new(9)), 1);

    let possible_rolls = gen_possible_rolls();

    let mut p1_wins = 0;
    let mut p2_wins = 0;

    let mut player1_turn = true;
    loop {
        let mut new_universes = HashMap::new();
        for ((player1, player2), num_states) in universes {
            for (roll, count) in &possible_rolls {
                let mut p1_copy = player1;
                let mut p2_copy = player2;

                let player_to_move = if player1_turn {
                    &mut p1_copy
                } else {
                    &mut p2_copy
                };
                player_to_move.move_forward(roll);

                let new_count = count * num_states;
                if player_to_move.score >= 21 {
                    if player1_turn {
                        p1_wins += new_count;
                    } else {
                        p2_wins += new_count;
                    }
                } else {
                    if new_universes.contains_key(&(p1_copy, p2_copy)) {
                        *new_universes.get_mut(&(p1_copy, p2_copy)).unwrap() += new_count;
                    } else {
                        new_universes.insert((p1_copy, p2_copy), new_count);
                    }
                }
            }
        }

        universes = new_universes;

        // dbg!(&universes);

        player1_turn = !player1_turn;

        if universes.is_empty() {
            break;
        } 
        // println!("Universes: {}", universes.len());
    }

    p1_wins.max(p2_wins)
}

pub fn solve(_: Input) -> Solution {
    Solution::new(part1(), part2())
}
