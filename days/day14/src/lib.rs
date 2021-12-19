#![allow(unused_imports)]
use std::iter::FromIterator;
use std::{collections::HashMap, hash::Hash};
use std::slice::Windows;

use parse_display::FromStr;
use utils::{Solution, Input};

#[derive(FromStr, Debug)]
#[display("{from} -> {to}")]
struct Rule {
    from: String,
    to: String,
}

trait IncrOrInsert {
    fn inc_or_insert(&mut self, p: String, inc: i64);
}

impl IncrOrInsert for HashMap<String, i64> {
    fn inc_or_insert(&mut self, p: String, inc: i64) {
        if self.contains_key(&p) {
            *self.get_mut(&p).unwrap() += inc;
        } else {
            self.insert(p, inc);
        }
    }
}

struct Polymer {
    pairs: HashMap<String, i64>,
    first: String,
    last: String,
}

impl Polymer {
    pub fn new(s: &str) -> Polymer {
        let mut pairs = HashMap::new();
        for p in s.chars().collect::<Vec<_>>().windows(2) {
            pairs.inc_or_insert(p[0].to_string() + &p[1].to_string(), 1);
        }
        Polymer { pairs, first: s.chars().nth(0).unwrap().to_string(), 
            last: s.chars().nth(s.len() - 1).unwrap().to_string(), 
        }
    }

    pub fn iterate(&mut self, rules: &HashMap<String, String>) {
        let mut result = HashMap::new();

        for (p, count) in &self.pairs {
            let new_char = rules.get(p).unwrap().clone();

            result.inc_or_insert(p.chars().nth(0).unwrap().to_string() + &new_char, *count);
            result.inc_or_insert(new_char + &p.chars().nth(1).unwrap().to_string(), *count);
        }

        self.pairs = result;
    }

    pub fn find_min_max(&self) -> i64 {
        let mut map = HashMap::new();
        for (p, count) in &self.pairs {
            map.inc_or_insert(p.chars().nth(0).unwrap().to_string(), *count);
            map.inc_or_insert(p.chars().nth(1).unwrap().to_string(), *count);
        }

        *map.get_mut(&self.first).unwrap() += 1;
        *map.get_mut(&self.last).unwrap() += 1;

        (map.iter().max_by(|(_, lhs), (_, rhs)| lhs.cmp(rhs)).unwrap().1
        - map.iter().min_by(|(_, lhs), (_, rhs)| lhs.cmp(rhs)).unwrap().1) / 2
    }
}

#[cfg(test)]
mod test_polymer {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn iterate() {
        let mut poly = Polymer::new("NNCB");

        let mut rules = HashMap::new();
        rules.insert(Rule::from_str("CH -> B").unwrap().from, Rule::from_str("CH -> B").unwrap().to);
        rules.insert(Rule::from_str("HH -> N").unwrap().from, Rule::from_str("HH -> N").unwrap().to);
        rules.insert(Rule::from_str("CB -> H").unwrap().from, Rule::from_str("CB -> H").unwrap().to);
        rules.insert(Rule::from_str("NH -> C").unwrap().from, Rule::from_str("NH -> C").unwrap().to);
        rules.insert(Rule::from_str("HB -> C").unwrap().from, Rule::from_str("HB -> C").unwrap().to);
        rules.insert(Rule::from_str("HC -> B").unwrap().from, Rule::from_str("HC -> B").unwrap().to);
        rules.insert(Rule::from_str("HN -> C").unwrap().from, Rule::from_str("HN -> C").unwrap().to);
        rules.insert(Rule::from_str("NN -> C").unwrap().from, Rule::from_str("NN -> C").unwrap().to);
        rules.insert(Rule::from_str("BH -> H").unwrap().from, Rule::from_str("BH -> H").unwrap().to);
        rules.insert(Rule::from_str("NC -> B").unwrap().from, Rule::from_str("NC -> B").unwrap().to);
        rules.insert(Rule::from_str("NB -> B").unwrap().from, Rule::from_str("NB -> B").unwrap().to);
        rules.insert(Rule::from_str("BN -> B").unwrap().from, Rule::from_str("BN -> B").unwrap().to);
        rules.insert(Rule::from_str("BB -> N").unwrap().from, Rule::from_str("BB -> N").unwrap().to);
        rules.insert(Rule::from_str("BC -> B").unwrap().from, Rule::from_str("BC -> B").unwrap().to);
        rules.insert(Rule::from_str("CC -> N").unwrap().from, Rule::from_str("CC -> N").unwrap().to);
        rules.insert(Rule::from_str("CN -> C").unwrap().from, Rule::from_str("CN -> C").unwrap().to);

        poly.iterate(&rules);
        assert_eq!(poly.pairs, Polymer::new("NCNBCHB").pairs);

        poly.iterate(&rules);
        assert_eq!(poly.pairs, Polymer::new("NBCCNBBBCBHCB").pairs);

        poly.iterate(&rules);
        assert_eq!(poly.pairs, Polymer::new("NBBBCNCCNBBNBNBBCHBHHBCHB").pairs);

        poly.iterate(&rules);
        assert_eq!(poly.pairs, Polymer::new("NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB").pairs);
    }
}

pub fn solve(input: Input) -> Solution {
    let mut polymer = Polymer::new(input.raw.lines().next().unwrap());

    let mut rules = HashMap::new();
    for rule in  input.raw.lines().skip(2).map(|s| s.parse::<Rule>().unwrap()) {
        rules.insert(rule.from.clone(), rule.to);
    }

    for step in 0..10 {
        polymer.iterate(&rules);
    }
    let part1 = polymer.find_min_max();
    for step in 0..30 {
        polymer.iterate(&rules);
    }
    let part2 = polymer.find_min_max();

    Solution::new(part1, part2)
}
