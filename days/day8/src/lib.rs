#![allow(unused_imports)]
use std::{collections::{HashMap, HashSet}, usize, iter::FromIterator};

use utils::{Input, Solution};

fn decode(patterns: &Vec<String>) -> HashMap<char, char> {
    let mut result = HashMap::new();

    let chars = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];

    let one = patterns.iter().find(|s| s.len() == 2).unwrap();
    let seven = patterns.iter().find(|s| s.len() == 3).unwrap();
    let four = patterns.iter().find(|s| s.len() == 4).unwrap();
    let eight = patterns.iter().find(|s| s.len() == 7).unwrap();

    // 1 and 7 share all positions except 'a'. So the one character
    // that is in 7 but not in 1 must be the one corresponding to the 'a' position
    let maps_to_a = seven.chars().find(|c| !one.contains(&c.to_string())).unwrap();
    result.insert(maps_to_a, 'a');

    // 1 consists of the c and f position. 9 and 0 have the c position, but 6 does NOT
    let nine_six_zero: Vec<_> = patterns.iter().filter(|s| s.len() == 6).collect();
    // So the char of 1 that is in all of 6, 0 and 9 must be the f position
    let maps_to_f = one.chars().find(|c| nine_six_zero.iter().all(|s| s.contains(&c.to_string()))).unwrap();
    result.insert(maps_to_f, 'f');

    // The other char of 1 must then of course be the c position
    let maps_to_c = one.chars().find(|c| *c != maps_to_f).unwrap();
    result.insert(maps_to_c, 'c');

    let two_three_five: Vec<_> = patterns.iter().filter(|s| s.len() == 5).collect();

    // 0,6,9 and 2,3,5 share exactly two common position: a and g.
    // Since we already know what char maps to a, we can use this to figure out which one is g 
    let maps_to_g = chars.iter()
        .find(|c| {
            nine_six_zero.iter().chain(two_three_five.iter()).all(|s| {
                s.contains(&c.to_string()) && **c != maps_to_a
            })
        }).unwrap();
    result.insert(*maps_to_g, 'g');

    // 0,6 and 9 share the positions a b f and g.
    // Since we already have decoded a f and g, we can now figure out b
    let maps_to_b = chars.iter()
        .find(|c| {
            nine_six_zero.iter().all(|s| {
                s.contains(&c.to_string()) && **c != maps_to_a && **c != maps_to_f && **c != *maps_to_g
            })
        }).unwrap();
    result.insert(*maps_to_b, 'b');

    // 4 consists of b, c, d, f. We already now b c and f, so we can figure out d now
    let maps_to_d = four.chars().find(|c| *c != *maps_to_b && *c != maps_to_c && *c != maps_to_f).unwrap();
    result.insert(maps_to_d, 'd');

    // We now have 6 of 7 positions, so just find the remaining one that we haven't decoded yet
    let maps_to_e = chars.iter().find(|c| !result.contains_key(c)).unwrap();
    result.insert(*maps_to_e, 'e');

    result
}

fn str_to_char_set(s: &str) -> HashSet<char> {
    HashSet::from_iter(s.chars())
}

fn segment_string_to_number(segment_str: &str) -> i64 {
    let chars = str_to_char_set(segment_str);
    if chars == str_to_char_set("abcefg") {
        return 0
    } else if chars == str_to_char_set("cf") {
        return 1
    } else if chars == str_to_char_set("acdeg") {
        return 2
    } else if chars == str_to_char_set("acdfg") {
        return 3
    } else if chars == str_to_char_set("bcdf") {
        return 4
    } else if chars == str_to_char_set("abdfg") {
        return 5
    } else if chars == str_to_char_set("abdefg") {
        return 6
    } else if chars == str_to_char_set("acf") {
        return 7
    } else if chars == str_to_char_set("abcdefg") {
        return 8
    } else if chars == str_to_char_set("abcdfg") {
        return 9
    }
    unreachable!()
}

fn decode_numbers(numbers: &[String], map: HashMap<char, char>) -> i64 {
    numbers.iter().map(|number| {
        let mapped_number: String = number.chars().map(|c| map.get(&c).unwrap()).collect();
        segment_string_to_number(&mapped_number)
    }).fold("".to_string(), |acc, n| {
        acc + &n.to_string()
    }).parse().unwrap()
}

pub fn solve(input: Input) -> Solution {
    let output: Vec<Vec<String>> = input.raw.lines()
        .map(|l| l.split('|').nth(1).unwrap().to_string())
        .map(|s| s.split_whitespace().map(|s2| s2.to_string()).collect())
        .collect();

    let inputs: Vec<Vec<String>> = input.raw.lines()
        .map(|l| l.split('|').nth(0).unwrap().to_string())
        .map(|s| s.split_whitespace().map(|s2| s2.to_string()).collect())
        .collect();

    let decoded: Vec<_> = inputs.iter()
        .map(|vec| decode(vec))
        .collect();

    let part1: usize = output.iter()
        .map(|vec| vec.iter().filter(|s| s.len() == 2 || s.len() == 4 || s.len() == 3 || s.len() == 7).count())
        .sum();

    let part2: i64 = output.iter().zip(decoded)
        .map(|(vec, map)| decode_numbers(vec, map))
        .sum();

    Solution::new(part1, part2)
}
