#![allow(unused_imports)]
use utils::{Solution, Input};

trait GetBit {
    fn get_bit(&self, i: &u32) -> usize;
}

impl GetBit for i64 {
    fn get_bit(&self, i: &u32) -> usize {
        let mask = 2i64.pow(*i);
        if (self & mask) == mask { 1 } else { 0 }
    }
}

fn most_common(numbers: &[i64], pos: &u32) -> Option<i64> {
    let count = numbers.iter()
        .filter(|n| n.get_bit(pos) == 1)
        .count();

    println!("len {} count {} len/2 {}", numbers.len(), count, numbers.len() / 2);
    match numbers.len() % 2 {
        0 => 
            if count > numbers.len() / 2 {
                Some(1)
            } else if count < numbers.len() / 2 {
                Some(0)
            } else { None },
        1 => if count > numbers.len() / 2 {
            Some(1)
        } else {
            Some(0)
        }
        _ => unreachable!()
    }
}

fn least_common(numbers: &[i64], pos: &u32) -> Option<i64> {
    match most_common(numbers, pos) {
        Some(0) => Some(1),
        Some(1) => Some(0),
        None => None,
        _ => unreachable!(),
    }
}

fn part1(numbers: &[i64], size: &u32) -> i64 {
    let mut gamma: String = "".to_string();
    for i in 0..*size {
        gamma = most_common(numbers, &i).unwrap().to_owned().to_string() + &gamma;
    }

    let gamma_i = i64::from_str_radix(&gamma, 2).unwrap();
    let epsilon = gamma_i ^ 0b10i64.pow(*size) - 1;

    gamma_i * epsilon
}

fn part2(num_slice: &[i64], size: &u32) -> i64 {
    let mut numbers: Vec<i64> = num_slice.to_vec();
    let mut pos = size - 1;
    let mut oxygen = 0;
    loop {
        let most = most_common(&numbers, &pos);
        dbg!(most);

        numbers = numbers.iter()
            .filter(|n| {
                println!("{:b}, {}, {}", n, pos, n.get_bit(&pos));
                match most {
                    None => n.get_bit(&pos) == 1,
                    Some(m) => m as usize == n.get_bit(&pos),
                }
            })
            .cloned()
            .collect();
        dbg!(&numbers);

        if numbers.len() == 1 {
            oxygen = numbers[0];
            break;
        }

        pos -= 1;
    }

    let mut co2 = 0;
    pos = size - 1;
    numbers = num_slice.to_vec();
    loop {
        let least = least_common(&numbers, &pos);
        dbg!(least);

        numbers = numbers.iter()
            .filter(|n| {
                println!("{:b}, {}, {}", n, pos, n.get_bit(&pos));
                match least {
                    None => n.get_bit(&pos) == 0,
                    Some(m) => m as usize == n.get_bit(&pos),
                }
            })
            .cloned()
            .collect();
        dbg!(&numbers);

        if numbers.len() == 1 {
            co2 = numbers[0];
            break;
        }

        pos -= 1;
    }

    co2 * oxygen
}

pub fn solve(input: Input) -> Solution {
    let numbers: Vec<i64> = input.parse::<String>().iter().map(|s| i64::from_str_radix(s, 2).unwrap()).collect();
    let size = input.raw.lines().next().unwrap().len() as u32;

    Solution::new(part1(&numbers, &size), part2(&numbers, &size))
}
