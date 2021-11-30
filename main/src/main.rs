fn main() {
    let days: Vec<fn(&str) -> utils::Solution> = vec![
        day1::solve,
        day2::solve,
        day3::solve,
        day4::solve,
        day5::solve,
        day6::solve,
        day7::solve,
        day8::solve,
        day9::solve,
        day10::solve,
        day11::solve,
        day12::solve,
        day13::solve,
        day14::solve,
        day15::solve,
        day16::solve,
        day17::solve,
        day18::solve,
        day19::solve,
        day20::solve,
        day21::solve,
        day22::solve,
        day23::solve,
        day24::solve,
        day25::solve,
    ];

    let args: Vec<String> = std::env::args().collect();

    let day = args[1].parse::<usize>().unwrap();
    let solution = days[day - 1];


    let default_input = format!("input/{}.in", day);
    let input_file = args.get(2).unwrap_or(&default_input);

    let solution = solution(&input_file);

    println!("Part 1 solution: {}", solution.part1);
    println!("Part 2 solution: {}", solution.part2);
}
