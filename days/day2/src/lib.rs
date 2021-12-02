use parse_display::{Display, FromStr};
use utils::get_lines;

#[derive(Display, FromStr)]
enum Instruction {
    #[display("up {0}")]
    Up(i64),
    #[display("down {0}")]
    Down(i64),
    #[display("forward {0}")]
    Forward(i64),
}

fn part1(instructions: &[Instruction]) -> i64 {
    let (hor, depth) = instructions
        .iter()
        .fold((0, 0), |(mut hor, mut depth), instr| {
            match instr {
                Instruction::Down(x) => depth += x,
                Instruction::Up(x) => depth -= x,
                Instruction::Forward(x) => hor += x,
            }
            (hor, depth)
        });
    hor * depth
}

fn part2(instructions: &[Instruction]) -> i64 {
    let (hor, depth, _) =
        instructions
            .iter()
            .fold((0, 0, 0), |(mut hor, mut depth, mut aim), instr| {
                match instr {
                    Instruction::Down(x) => aim += x,
                    Instruction::Up(x) => aim -= x,
                    Instruction::Forward(x) => {
                        hor += x;
                        depth += aim * x;
                    }
                }
                (hor, depth, aim)
            });
    hor * depth
}

pub fn solve(_input_file: &str) -> utils::Solution {
    let instructions: Vec<Instruction> = get_lines(_input_file)
        .iter()
        .map(|l| l.parse().unwrap())
        .collect();

    utils::Solution::new(part1(&instructions), part2(&instructions))
}
