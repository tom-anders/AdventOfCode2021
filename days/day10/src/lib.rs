#![allow(unused_imports)]
use utils::{Input, Solution};

trait Line {
    fn reduce(&self) -> String;
    fn is_corrupted(&self) -> Option<usize>;
    fn complete_score(&self) -> usize;
}

impl Line for String {
    fn reduce(&self) -> String {
        let mut s = self.clone();
        // Remove direct pairs () {} [] and <> until none remain
        loop {
            let pos = s.chars().enumerate().find(|(i, c)| {
                if *i == s.len() - 1 {
                    return false;
                }

                s.chars().nth(i + 1)
                    == match c {
                        '(' => Some(')'),
                        '<' => Some('>'),
                        '[' => Some(']'),
                        '{' => Some('}'),
                        _ => None,
                    }
            });

            match pos {
                Some((i, _)) => {
                    s.remove(i);
                    s.remove(i);
                }
                None => break,
            };
        }

        s
    }

    fn is_corrupted(&self) -> Option<usize> {
        // If the result now still contains a closing char, it's corrupted by that char
        // Otherwise it's either valid or incomplete
        self.reduce().chars().find_map(|c| match c {
            ')' => Some(3),
            ']' => Some(57),
            '}' => Some(1197),
            '>' => Some(25137),
            _ => None,
        })
    }

    fn complete_score(&self) -> usize {
        self.reduce().chars().rev().fold(0, |score, c| {
            score * 5
                + match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => unreachable!("Got unexpected {}", c),
                }
        })
    }
}

pub fn solve(input: Input) -> Solution {
    let lines: Vec<String> = input.parse();

    let mut scores: Vec<_> = lines
        .iter()
        .filter(|l| l.is_corrupted().is_none())
        .map(|l| l.complete_score())
        .collect();
    scores.sort_unstable();

    Solution::new(
        lines.iter().filter_map(|l| l.is_corrupted()).sum::<usize>(),
        scores[scores.len() / 2],
    )
}
