use {{crate_name}}::{solve_part1, solve_part2, INPUT};
use std::env;

fn part1() {
    let result = solve_part1(INPUT);
    println!("Part 1 => {:?}", result);
}

fn part2() {
    let result = solve_part2(INPUT);
    println!("Part 2 => {:?}", result);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(s) = args.get(1) {
        match s.as_str() {
            "1" | "part1" => {
                part1();
                return;
            }
            "2" | "part2" => {
                part2();
                return;
            }
            _ => (),
        }
    };
    part1();
    part2();
}
