use utils::get_lines;

type Pos = num::Complex<i64>;

fn advance(line: &String, pos: &mut Pos) {
    let split: Vec<_> = line.split(" ").collect();
    let dir = match split[0] {
        "forward" => Pos::new(1, 0),
        "down" => Pos::new(0, 1),
        "up" => Pos::new(0, -1),
        _ => unreachable!(),
    };
    let inc: i64 = split[1].parse().unwrap();

    *pos += dir * inc;
}

fn advance_b(line: &String, pos: &mut Pos, aim: &mut i64) {
    let split: Vec<_> = line.split(" ").collect();
    let inc: i64 = split[1].parse().unwrap();
    match split[0] {
        "down" => *aim += inc,
        "up" => *aim -= inc,
        "forward" => {
            *pos += inc + Pos::new(0, 1) * *aim * inc;
        }
        _ => unreachable!(),
    };
}

pub fn solve(_input_file: &str) -> utils::Solution {
    let mut pos = Pos::new(0, 0);
    for line in get_lines(_input_file) {
        advance(&line, &mut pos);
    }
    let part1 = pos.re * pos.im;

    pos = Pos::new(0, 0);
    let mut aim = 0;
    for line in get_lines(_input_file) {
        advance_b(&line, &mut pos, &mut aim);
    }
    let part2 = pos.re * pos.im;

    utils::Solution::new(part1, part2)
}
